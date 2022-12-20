use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Path};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use std::sync::Arc;
use tracing::error;

use crate::auth::{AuthConfiguration, Providers};
use crate::config::Application;
use ntt_core::io::auth::auth::NttAuthState;

use crate::errors::NttBackendResult;
pub fn configure(config: &mut web::ServiceConfig) {
    config.service(login).service(call_back);
}

#[get("/login/{provider}")]
pub async fn login(
    provider: Path<Providers>,
    application: Data<Application>,
) -> NttBackendResult<impl Responder> {
    match provider.as_ref() {
        Providers::Github => {
            if let Some(github) = &application.auth.github {
                let auth_state = NttAuthState::new(
                    github.client_id.clone(),
                    github.client_secret.clone(),
                    format!("{}/auth/login/Github/callback", application.url),
                )?;

                Ok(HttpResponse::build(StatusCode::FOUND)
                    .append_header((LOCATION, auth_state.auth_url.as_str()))
                    .finish())
            } else {
                // Invalid provider
                Ok(HttpResponse::Found().header(LOCATION, "/").finish())
            }
        }
    }
}
#[derive(Debug, Deserialize)]
pub struct ErrorCallback {
    error: String,
    error_description: String,
    error_uri: String,
    state: String,
}
#[derive(Debug, Deserialize)]
pub struct CallBack {
    code: String,
}
#[get("/login/{provider}/callback")]
pub async fn call_back(
    config: Data<Application>,
    provider: Path<Providers>,
    query: HttpRequest,
) -> NttBackendResult<impl Responder> {
    let query = query.query_string();
    let query = serde_qs::from_str::<CallBack>(query);
    if let Ok(query) = query {
        Ok(HttpResponse::Found().header(LOCATION, "/").finish())
    } else {
        let query = serde_qs::from_str::<ErrorCallback>(query);
        if let Ok(query) = query {
            error!("Error: {}", query.error);
            error!("Error Description: {}", query.error_description);
            error!("Error URI: {}", query.error_uri);
            error!("State: {}", query.state);
        }
        // TODO error handling
        Ok(HttpResponse::Found().header(LOCATION, "/").finish())
    }
}
