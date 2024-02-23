use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde::Deserialize;
use std::env;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::new().expect("Failed to setup settings");
}

#[derive(Deserialize, Debug)]
pub struct Application {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub struct Database {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub namespace: String,
    pub dbname: String,
}

#[derive(Deserialize, Debug)]
pub struct Logger {
    pub level: String,
}

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub environment: String,
    pub application: Application,
    pub database: Database,
    pub logger: Logger,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let app_environment = env::var("APP_ENVIRONMENT").unwrap_or_else(|_| "development".into());

        let builder = Config::builder()
            .add_source(
                File::with_name(&format!("configuration/{app_environment}")).required(false),
            )
            .add_source(Environment::default().prefix("APP").separator("_"));

        builder.build()?.try_deserialize()
    }
}

#[test]
fn test_config() -> Result<(), config::ConfigError> {
    let _ = temp_env::with_vars(
        [
            ("APP_ENVIRONMENT", Some("production")),
            ("APP_DATABASE_USERNAME", Some("test")),
            ("APP_DATABASE_PASSWORD", Some("test")),
            ("APP_DATABASE_NAMESPACE", Some("test")),
            ("APP_DATABASE_DBNAME", Some("test")),
        ],
        || Settings::new().expect("Failed to load settings"),
    );
    Ok(())
}
