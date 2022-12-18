use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct NttGitHubAuthConfiguration {
    pub client_id: String,
    pub client_secret: String
}

#[derive(Debug, Deserialize, Clone)]
pub struct NttServerConfiguration {
    pub address: String,
    pub port: u16
}

#[derive(Debug, Deserialize, Clone)]
pub struct NttBackendConfiguration {
    pub auth: NttGitHubAuthConfiguration,
    pub server: NttServerConfiguration
}