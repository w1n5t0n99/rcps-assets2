use actix_web::dev::Server;
use actix_web::middleware::ErrorHandlers;
use actix_web::{web, App, HttpServer};
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_session::config::PersistentSession;
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use actix_web_grants::GrantsMiddleware;
use actix_web_lab::middleware::from_fn;
use actix_files;
use actix_web::cookie::{Key, time::Duration};
use reqwest::StatusCode;
use sea_orm::{DatabaseConnection, ConnectOptions, Database};
use secrecy::{Secret, ExposeSecret};
use tracing_actix_web::TracingLogger;
use std::net::TcpListener;

use crate::configuration::{DatabaseSettings, Settings};
use crate::auth::{reject_anonymous_users, extract_user_permissions, check_user_password_status};
use crate::routes::*;


pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let db_conn = get_database_connection(&configuration.database).await;

        let address = format!("{}:{}", configuration.application.host, configuration.application.port);
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            db_conn,
            configuration.application.base_url,
            configuration.application.hmac_secret,
        )
        .await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub async fn get_database_connection(configuration: &DatabaseSettings) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(configuration.get_connection_string());

    opt.sqlx_logging(true)
        .sqlx_logging_level(tracing::log::LevelFilter::Info)
        .acquire_timeout(std::time::Duration::from_secs(2));

    Database::connect(opt).await.expect("Could not connect to database")
}


async fn run(
    listener: TcpListener,
    db_connection: DatabaseConnection,
    _base_url: String,
    hmac_secret: Secret<String>,
) -> Result<Server, anyhow::Error> {
    let db_connection = web::Data::new(db_connection);
    let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    let message_framework = FlashMessagesFramework::builder(message_store).build();
   
    let server = HttpServer::new(move || {
        App::new()
            .wrap(message_framework.clone())
            .wrap(SessionMiddleware::builder(CookieSessionStore::default(), secret_key.clone())
                .session_lifecycle(PersistentSession::default().session_ttl(Duration::hours(8)))
                .build()
            )
            .wrap(TracingLogger::default())
            .wrap(
                ErrorHandlers::new()
                    .handler(StatusCode::BAD_REQUEST, errors::render_400)
                    .handler(StatusCode::FORBIDDEN, errors::render_403)
                    .handler(StatusCode::NOT_FOUND, errors::render_404)
                    .handler(StatusCode::INTERNAL_SERVER_ERROR, errors::render_500)
            )
            .app_data(db_connection.clone())
            .configure(init)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check::health_check);
    cfg.service(actix_files::Files::new("/static", "./app/static"));

    cfg.service(
        web::scope("/groups")
            .wrap(GrantsMiddleware::with_extractor(extract_user_permissions))
            .wrap(from_fn(check_user_password_status))
            .wrap(from_fn(reject_anonymous_users))
            .service(asset_items::get_asset_items)
        );

    cfg.service(
        web::scope("/users")
            .service(login::view_sign_in)
            .service(login::post_sign_in)
            .service(web::scope("/{id}")
                .wrap(from_fn(reject_anonymous_users))
                .service(password::view_edit_password)
                .service(password::post_edit_password)
                .service(logout::sign_out)
            )
        );

}