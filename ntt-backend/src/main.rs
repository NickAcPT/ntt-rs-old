use std::fs;
use std::sync::Arc;

use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use tracing::info;
use tracing_subscriber::FmtSubscriber;

use crate::config::NttBackendConfiguration;

mod config;
mod endpoints;
pub(crate) mod errors;
mod io;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let fmt_subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(fmt_subscriber)
        .expect("setting tracing default failed");

    let config: Arc<NttBackendConfiguration> =
        Arc::new(toml::from_str(fs::read_to_string("config.toml")?.as_str())?);

    let server_config = &config.server;
    let address = server_config.address.to_owned();
    let port = server_config.port;

    info!("Starting web server at {}:{}", address, port);

    HttpServer::new(move || {
        App::new()
            .service(web::scope("auth").service(endpoints::auth::login))
            .app_data(Data::new(config.clone()))
    })
    .bind((address, port))?
    .run()
    .await
}
