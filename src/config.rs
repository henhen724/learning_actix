// Loads the configuration of the server into a
use config::ConfigError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: i32,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
}

impl Config {
    pub fn build_config() -> Result<Self, ConfigError> {
        let mut config = config::Config::new();
        config.merge(config::File::with_name("config.toml").required(false))?;
        config.merge(config::Environment::default())?;

        config.try_into()
    }
}
