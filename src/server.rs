use std::net::SocketAddr;

use anyhow::Ok;
use axum::Router;
use tokio::net::TcpListener;

use crate::{app::AppState, config::server::ServerConfig};
pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Self { config }
    }
    pub async fn start(&self, state: AppState, router: Router<AppState>) -> anyhow::Result<()> {
        let router = self.build_router(state, router);
        let port = self.config.port();
        let address = format!("0.0.0.0:{port}");
        let listener = TcpListener::bind(address.clone()).await?;
        tracing::info!("Server listening on  http://{}", address);
        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;
        Ok(())
    }

    fn build_router(&self, state: AppState, router: Router<AppState>) -> axum::Router {
        axum::Router::new().merge(router).with_state(state.clone())
    }
}
