use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{get, HttpResponse, Responder};
use std::sync::Arc;

use ntt_core::io::auth::auth::NttAuthState;

use crate::config::NttBackendConfiguration;
use crate::errors::NttBackendResult;

#[get("/login")]
pub async fn login(config: Data<Arc<NttBackendConfiguration>>) -> NttBackendResult<impl Responder> {
    let auth_configuration = &config.auth;
    let auth_state = NttAuthState::new(
        auth_configuration.client_id.clone(),
        auth_configuration.client_secret.clone(),
    )?;

    Ok(HttpResponse::build(StatusCode::FOUND)
        .append_header((LOCATION, auth_state.auth_url.as_str()))
        .finish())
}
