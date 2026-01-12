use std::{net::SocketAddr, time::Duration};

use anyhow::Ok;
use axum::{
    Router,
    extract::{DefaultBodyLimit, Request},
    http::{StatusCode, method, request},
};
use bytesize::ByteSize;
use sea_orm::prelude::Time;
use tokio::net::TcpListener;
use tower_http::{
    cors::Any,
    normalize_path::NormalizePathLayer,
    timeout::TimeoutLayer,
    trace::{self, DefaultOnResponse, TraceLayer},
};

use crate::{app::AppState, app::latency::LatencyOnResponse, config::server::ServerConfig};
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
        let timeout_layer =
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(120));
        let body_limit_layer = DefaultBodyLimit::max(ByteSize::mib(10).as_u64() as usize);
        let cors_layer = tower_http::cors::CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods(vec![
                method::Method::GET,
                method::Method::POST,
                method::Method::PUT,
                method::Method::DELETE,
            ])
            .allow_headers(tower_http::cors::Any)
            .allow_credentials(false)
            .max_age(Duration::from_secs(3600 * 12));
        // 末尾的 / 去掉
        let normalize_layer = NormalizePathLayer::trim_trailing_slash();

        let trace_layer = TraceLayer::new_for_http()
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
            .layer(timeout_layer)
            .layer(body_limit_layer)
            .layer(trace_layer)
            .layer(cors_layer)
            .layer(normalize_layer)
            .with_state(state)
    }
}
