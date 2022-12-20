use crate::auth::AuthConfiguration;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct NttServerConfiguration {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct NttBackendConfiguration {
    pub auth: AuthConfiguration,
    pub server: NttServerConfiguration,
}
