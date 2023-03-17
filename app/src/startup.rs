use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web_lab::middleware::from_fn;
use actix_files;
use jsonwebtoken::{DecodingKey, EncodingKey};
use oso::{Oso, PolarClass};
use sea_orm::{DatabaseConnection, ConnectOptions, Database};
use secrecy::{Secret, ExposeSecret};
use tracing_actix_web::TracingLogger;
use std::net::TcpListener;

use crate::configuration::{DatabaseSettings, Settings};
use crate::auth::jwt_middleware::reject_invalid_jwt;
use crate::auth::authorize::Authorize;
use crate::auth::ApiClient;
use crate::api;
use crate::error_responses::e400;
use ::entity::user;


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
    let decoding_key = web::Data::new(DecodingKey::from_secret(hmac_secret.expose_secret().as_bytes()));
    let encoding_key = web::Data::new(EncodingKey::from_secret(hmac_secret.expose_secret().as_bytes()));

    // Set up authorization
    let mut oso = Oso::new();
    oso.register_class(user::Model::get_polar_class_builder().name("User").build())?;
    oso.register_class(ApiClient::get_polar_class_builder().name("Client").build())?;
    oso.load_files(vec!["./app/polar/users_authorization.polar"])?;

    let authorize = web::Data::new(Authorize::new(oso));

    let jsonconfig = web::JsonConfig::default().error_handler(|_err, _req| {
        e400("error", "Invalid client-side data (JSON)", "ValidationError")
    });
   
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(db_connection.clone())
            .app_data(encoding_key.clone())
            .app_data(decoding_key.clone())
            .app_data(authorize.clone())
            .app_data(jsonconfig.clone())
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
        .service(api::users::user_delete::delete_user_handler)
        .service(api::users::user_create::create_user_handler);

    let scope = web::scope("/api")
        .service(api::health_check::health_checker)
        .service(account_scope)
        .service(session_scope)
        .service(users_scope);

    cfg.service(scope); 
}