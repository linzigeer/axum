use crate::app::AppState;
use crate::configs::ServerConfig;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Debug, Clone)]
pub struct Server {
    server: &'static ServerConfig,
}

impl Server {
    pub fn new(server: &'static ServerConfig) -> Self {
        Server { server }
    }

    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let router = self.build_router(state, router);

        let ip = self.server.get_server_ip();
        let port = self.server.get_server_port();
        let tcp_listener = TcpListener::bind(format!("{ip}:{port}")).await?;
        tracing::info!("Server starting at {}", tcp_listener.local_addr()?);
        axum::serve(
            tcp_listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        tracing::info!("Server started successfully");

        Ok(())
    }

    fn build_router(&self, state: AppState, router: Router<AppState>) -> Router {
        Router::new().merge(router).with_state(state)
    }
}
