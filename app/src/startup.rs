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
use jsonwebtoken::{DecodingKey, EncodingKey};
use reqwest::StatusCode;
use sea_orm::{DatabaseConnection, ConnectOptions, Database};
use secrecy::{Secret, ExposeSecret};
use tracing_actix_web::TracingLogger;
use std::net::TcpListener;

use crate::configuration::{DatabaseSettings, Settings};
use crate::auth::jwt_middleware::reject_invalid_jwt;
use crate::api;


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
    //let secret_key = Key::from(hmac_secret.expose_secret().as_bytes());
    //let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    //let message_framework = FlashMessagesFramework::builder(message_store).build();

    let db_connection = web::Data::new(db_connection);
    let decoding_key = web::Data::new(DecodingKey::from_secret(hmac_secret.expose_secret().as_bytes()));
    let encoding_key = web::Data::new(EncodingKey::from_secret(hmac_secret.expose_secret().as_bytes()));
   
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(db_connection.clone())
            .app_data(encoding_key.clone())
            .app_data(decoding_key.clone())
            .configure(init)
    })
    .listen(listener)?
    .run();
    Ok(server)
}

fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(actix_files::Files::new("/static", "./app/static"));

    let account_scope = web::scope("/account")
        .service(api::account::register::register_account_handler);

    let session_scope = web::scope("/session")
        .service(api::session::login::login_user_handler);

    let users_scope = web::scope("/users")
        .wrap(from_fn(reject_invalid_jwt))
        .service(api::users::user_update::update_user_handler)
        .service(api::users::user_details::get_user_details_handler)
        .service(api::users::users_get::gets_users_handler)
        .service(api::users::user_create::create_user_handler);

    let scope = web::scope("/api")
        .service(api::health_check::health_checker)
        .service(account_scope)
        .service(session_scope)
        .service(users_scope);

    cfg.service(scope); 
}