use serde::Deserialize;

pub mod middleware;

/// The configuration for the authentication system.
/// Future versions could contain more options.
#[derive(Debug, Deserialize, Clone)]
#[serde(tag = "type", content = "content")]
#[non_exhaustive]
pub enum AuthConfiguration {
    /// Uses the Github OAuth2 authentication system.
    GitHub {
        client_id: String,
        client_secret: String,
    },
}
