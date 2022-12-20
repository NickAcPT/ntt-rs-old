use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub mod middleware;
pub mod session;
pub mod web;
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, ToSchema)]
pub enum Providers {
    Github,
}
/// The configuration for the authentication system.
/// Future versions could contain more options.
#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct AuthConfiguration {
    pub github: Option<GithubAuthConfiguration>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct GithubAuthConfiguration {
    pub client_id: String,
    pub client_secret: String,
}
