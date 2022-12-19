use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{get, HttpResponse, Responder};
use std::sync::Arc;

use ntt_core::io::auth::auth::NttAuthState;

use crate::config::{AuthConfiguration, NttBackendConfiguration};
use crate::errors::NttBackendResult;

#[get("/login")]
pub async fn login(config: Data<AuthConfiguration>) -> NttBackendResult<impl Responder> {
    match config.as_ref() {
        AuthConfiguration::GitHub{ client_id, client_secret} => {
            let auth_state =
                NttAuthState::new(client_id.clone(), client_secret.clone())?;

            Ok(HttpResponse::build(StatusCode::FOUND)
                .append_header((LOCATION, auth_state.auth_url.as_str()))
                .finish())
        }
    }
}
