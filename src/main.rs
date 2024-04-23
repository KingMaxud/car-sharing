use axum::Router;
use tracing::log::debug;

use crate::config::config;
use crate::routes::app_router;

mod config;
mod error;
mod handlers;
mod infra;
mod middlewares;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let config = config().await;

    env_logger::init();

    let app = Router::new().nest("/api", app_router(config.db_url()).await);

    let host = config.server_host();
    let port = config.server_port();

    let address = format!("{}:{}", host, port);

    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    debug!("LISTENING on {:?}\n", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
