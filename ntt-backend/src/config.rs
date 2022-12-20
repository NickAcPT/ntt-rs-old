use crate::auth::AuthConfiguration;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NttServerConfiguration {
    pub address: String,
    pub port: u16,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct Application {
    pub url: String,
    pub approve_everyone: bool,
    pub auth: AuthConfiguration,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DatabaseConfiguration {
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
}
impl Default for DatabaseConfiguration {
    fn default() -> Self {
        Self {
            host: "localhost:5123".to_string(),
            user: "postgres".to_string(),
            password: "postgrespw".to_string(),
            database: "ntt-rs".to_string(),
        }
    }
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NttBackendConfiguration {
    pub server: NttServerConfiguration,
    pub database: DatabaseConfiguration,
    pub application: Application,
}

impl Default for NttBackendConfiguration {
    fn default() -> Self {
        Self {
            server: NttServerConfiguration {
                address: "0.0.0.0".to_string(),
                port: 5234,
            },
            database: Default::default(),
            application: Application {
                url: "http://localhost:5234".to_string(),
                approve_everyone: true,
                auth: AuthConfiguration::default(),
            },
        }
    }
}
