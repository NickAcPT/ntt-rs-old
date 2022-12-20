use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use crate::auth::middleware::HandleSession;
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use sqlx::migrate;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::config::NttBackendConfiguration;

pub mod api;
pub mod auth;
mod config;
pub(crate) mod errors;
mod io;
use sqlx::postgres::PgPoolOptions;
use tracing::field::debug;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let fmt_subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();
    tracing::subscriber::set_global_default(fmt_subscriber)
        .expect("setting tracing default failed");
    let config = PathBuf::from("config.toml");
    if !config.exists() {
        let configuration = NttBackendConfiguration::default();
        let toml = toml::to_string(&configuration).expect("Failed to serialize configuration");
        fs::write(config, toml)?;
        println!("Created default configuration file");
        return Ok(());
    }
    let config: NttBackendConfiguration =
        toml::from_str(fs::read_to_string("config.toml")?.as_str())?;

    let server_config = config.server;
    let address = server_config.address;
    let port = server_config.port;

    let mut builder = PgPoolOptions::new();
    let pool = builder
        .max_connections(5)
        .connect(
            format!(
                "postgres://{}:{}@{}/{}",
                config.database.user,
                config.database.password,
                config.database.host,
                config.database.database
            )
            .as_str(),
        )
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    {
        let mut connection = pool.acquire().await.map_err(|e| {
            std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to acquire database connection: {}", e),
            )
        })?;
        debug!("Migrating database");
        migrate!("../migrations")
            .run(&mut connection)
            .await
            .expect("Failed to run migrations");
    }
    info!("Starting web server at {}:{}", address, port);
    let database = Data::new(pool);
    let application = Data::new(config.application);

    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .app_data(application.clone())
            .service(web::scope("auth").configure(auth::web::configure))
            .service(web::scope("api").wrap(HandleSession(false)))
            .service(web::scope("").wrap(HandleSession(true)))
    })
    .bind((address, port))?
    .run()
    .await
}
