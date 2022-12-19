use serde::Deserialize;
use crate::auth::AuthConfiguration;


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
