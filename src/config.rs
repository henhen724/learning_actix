// Loads the configuration of the server into a
use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub dbname: String,
}

#[derive(Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
}

impl Config {
    pub fn build_config() -> Result<Self, ConfigError> {
        let mut config = config::Config::new();
        config.merge(config::File::with_name("config.toml").required(false))?;
        config.merge(config::Environment::default())?;

        config.try_into()
    }
}
