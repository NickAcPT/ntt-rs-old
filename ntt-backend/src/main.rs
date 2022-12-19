use std::fs;
use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use crate::auth::middleware::HandleSession;

use crate::config::NttBackendConfiguration;

pub mod auth;
mod config;
mod endpoints;
pub(crate) mod errors;
mod io;
pub mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let fmt_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(fmt_subscriber)
        .expect("setting tracing default failed");

    let config: NttBackendConfiguration =
        toml::from_str(fs::read_to_string("config.toml")?.as_str())?;

    let server_config = config.server;
    let address = server_config.address;
    let port = server_config.port;

    info!("Starting web server at {}:{}", address, port);
    let auth = Data::new(config.auth);
    HttpServer::new(move || {
        App::new()
            .app_data(auth.clone())
            .service(web::scope("auth").service(endpoints::auth::login))
            .service(web::scope("api").wrap(HandleSession(false)))
            .service(web::scope("").wrap(HandleSession(true)))
    })
    .bind((address, port))?
    .run()
    .await
}
