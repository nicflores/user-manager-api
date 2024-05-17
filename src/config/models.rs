use std::env;

use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

/// These are the environment variables that we anticipate will be used to configure the application.
/// ```
/// database_url: String
/// api_key: String
/// log_level: String
/// ```
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct AppConfig {
    database_url: String,
    api_key: String,
    log_level: String,
}

/// The application configuration is loaded from the config/local.toml file or the environment.
/// Currently these settings aren't being used, but they could be used to configure the application later on.
impl AppConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "local".into());
        Config::builder()
            .add_source(File::with_name(&format!("src/config/{}.toml", run_mode)).required(false))
            .add_source(Environment::with_prefix("APP"))
            .build()
            .unwrap()
            .try_deserialize()
    }
}
