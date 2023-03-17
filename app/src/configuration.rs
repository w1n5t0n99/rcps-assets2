use secrecy::{ExposeSecret, Secret};
use serde_aux::field_attributes::deserialize_number_from_string;
//use sqlx::postgres::{PgConnectOptions, PgSslMode};
//use sqlx::ConnectOptions;


#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub frontend_url: String,
    pub hmac_secret: Secret<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub username: String,
    pub password: Secret<String>,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn get_connection_string(&self) -> String {
        let ssl_mode = if self.require_ssl {
            "sslmode=require"
        } else {
            "sslmode=prefer"
        };
        
        format!("postgresql://{}:{}@{}:{}/{}?{}",
            &self.username,
            &self.password.expose_secret(),
            &self.host,
            self.port,
            &self.database_name,
            ssl_mode,
        )
    }

    pub fn get_test_connection_string(&self) -> String {
        // should generate a sqlite database for intergration testing
        todo!()
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("app").join("configuration");

    let settings = config::Config::builder()
        .add_source(config::File::from(configuration_directory.join("app.yaml")))
        // Add in settings from environment variables (with a prefix of APP and '__' as separator)
        // E.g. `APP_APPLICATION__PORT=5001 would set `Settings.application.port`
        .add_source(config::Environment::with_prefix("APP").prefix_separator("_").separator("__"))
        .build()?;
    
    settings.try_deserialize::<Settings>()
}

