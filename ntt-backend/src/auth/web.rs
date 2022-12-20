use actix_web::http::header::LOCATION;
use actix_web::http::StatusCode;
use actix_web::web::{Data, Path};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::de::{MapAccess, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::Formatter;
use std::sync::Arc;
use tracing::error;

use crate::auth::{AuthConfiguration, Providers};
use crate::config::Application;
use ntt_core::io::auth::auth::NttAuthState;

use crate::errors::NttBackendResult;
pub fn configure(config: &mut web::ServiceConfig) {
    config.service(login).service(call_back).service(providers);
}
#[utoipa::path(
get,
path = "/auth/providers",
responses(
(status = 200, description = "A list of providers supported by this instance", body = [Providers])))
]
#[get("/providers")]
pub async fn providers(application: Data<Application>) -> NttBackendResult<impl Responder> {
    let mut providers = Vec::with_capacity(1);
    if application.auth.github.is_some() {
        providers.push(Providers::Github);
    }
    Ok(HttpResponse::Ok().json(providers))
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
#[derive(Debug, PartialEq, Eq)]
pub enum CallBack {
    Success {
        code: String,
    },
    Error {
        error: String,
        error_description: String,
        error_uri: String,
        state: String,
    },
}
impl<'de> Deserialize<'de> for CallBack {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CallBackVisitor;
        impl<'de> Visitor<'de> for CallBackVisitor {
            type Value = CallBack;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A map of string to type")
            }
            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                if let Some((key, value)) = map.next_entry::<String, String>()? {
                    if key == "code" {
                        Ok(CallBack::Success { code: value })
                    } else if key == "error" {
                        let mut error_description = None;
                        let mut error_uri = None;
                        let mut state = None;
                        while let Some((key, value)) = map.next_entry::<String, String>()? {
                            match key.as_str() {
                                "error_description" => error_description = Some(value),
                                "error_uri" => error_uri = Some(value),
                                "state" => state = Some(value),
                                _ => {}
                            }
                        }
                        Ok(CallBack::Error {
                            error: value,
                            error_description: error_description.unwrap_or_default(),
                            error_uri: error_uri.unwrap_or_default(),
                            state: state.unwrap_or_default(),
                        })
                    } else {
                        Err(serde::de::Error::custom("Invalid key"))
                    }
                } else {
                    Err(serde::de::Error::custom("Invalid callback"))
                }
            }
        }
        deserializer.deserialize_map(CallBackVisitor)
    }
}
#[test]
pub fn test_call_back_deserializer() {
    let back = serde_qs::from_str::<CallBack>("code=1234").unwrap();
    assert_eq!(
        back,
        CallBack::Success {
            code: "1234".to_string()
        }
    );
    let back1 = serde_qs::from_str::<CallBack>(
        "error=1234&error_description=test&error_uri=test&state=1234",
    )
    .unwrap();
    assert_eq!(
        back1,
        CallBack::Error {
            error: "1234".to_string(),
            error_description: "test".to_string(),
            error_uri: "test".to_string(),
            state: "1234".to_string(),
        }
    );
}

#[get("/login/{provider}/callback")]
pub async fn call_back(
    config: Data<Application>,
    provider: Path<Providers>,
    query: HttpRequest,
    callback: web::Query<CallBack>,
) -> NttBackendResult<impl Responder> {
    match callback.into_inner() {
        CallBack::Success { .. } => {
            // TODO: Handle success
            println!("Success");
            Ok(HttpResponse::Found().header(LOCATION, "/").finish())
        }
        CallBack::Error {
            error,
            error_description,
            error_uri,
            state,
        } => {
            error!(
                "Error: {} Description: {} URI: {} State: {}",
                error, error_description, error_uri, state
            );
            Ok(HttpResponse::Found().header(LOCATION, "/").finish())
        }
    }
}
