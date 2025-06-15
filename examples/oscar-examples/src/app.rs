use crate::{configs, logger, server, db};
use axum::Router;
use sea_orm::DatabaseConnection;
#[derive(Debug, Clone)]
pub struct AppState {
   pub db: DatabaseConnection,
}

impl AppState {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

pub async fn run(router: Router<AppState>) -> anyhow::Result<()> {
    tracing::info!("Starting app server...");
    logger::init();
    let db = db::init().await?;
    let state = AppState::new(db);
    let server_config = configs::get_app_config().get_server_config();
    let server = server::Server::new(server_config);

    server.start(state, router).await
}
