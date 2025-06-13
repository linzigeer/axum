mod configs;
mod db;
mod logger;

use axum::{debug_handler, routing, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    logger::init();
    db::init().await?;
    let router = Router::new().route("/", routing::get(say_hello));
    let server_config = configs::get_app_config().server_config();
    let ip = server_config.get_server_ip();
    let port = server_config.get_server_port();
    let listener = TcpListener::bind(format!("{ip}:{port}")).await?;
    tracing::info!(
        "server is listening on: {:?}",
        listener.local_addr()?.to_string()
    );
    axum::serve(listener, router).await?;
    
    Ok(())
}

#[debug_handler]
async fn say_hello() -> &'static str {
    "Hello Axum!!!"
}
