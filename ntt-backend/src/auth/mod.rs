use serde::{Deserialize, Serialize};

pub mod middleware;
pub mod session;
pub mod web;
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
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
