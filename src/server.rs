use std::net::SocketAddr;

use anyhow::Ok;
use axum::{
    Router,
    extract::Request,
    http::{method, request},
};
use tokio::net::TcpListener;
use tower_http::trace::{self, DefaultOnResponse, TraceLayer};

use crate::{app::AppState, config::server::ServerConfig, latency::LatencyOnResponse};
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
        let layer = TraceLayer::new_for_http()
            .make_span_with(|request: &Request| {
                let method = request.method();
                let path = request.uri().path();
                let xid = xid::new();
                tracing::info_span!("api request",trace_id = %xid, method=%method, path= %path)
            })
            .on_request(())
            .on_failure(())
            .on_response(LatencyOnResponse);
        axum::Router::new()
            .merge(router)
            .layer(layer)
            .with_state(state)
    }
}
