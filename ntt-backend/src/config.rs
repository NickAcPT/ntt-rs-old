use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", content = "content")]
pub enum AuthConfiguration {
    GitHub{
        client_id: String,
        client_secret: String,
    },
}
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
