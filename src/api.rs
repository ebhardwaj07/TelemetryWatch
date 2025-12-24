use axum::{
    extract::State,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::sync::Arc;

use crate::db::Database;
use crate::metrics::Metrics;
use crate::middleware::metrics_middleware;

pub fn create_router(metrics: Arc<Metrics>, db: Arc<Database>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/metrics", get(get_metrics))
        .route("/api/v1/status", get(status))
        .layer(middleware::from_fn_with_state(
            metrics.clone(),
            metrics_middleware,
        ))
        .with_state(AppState { metrics, db })
}

#[derive(Clone)]
pub struct AppState {
    pub metrics: Arc<Metrics>,
    pub db: Arc<Database>,
}

async fn health() -> Response {
    (StatusCode::OK, "OK").into_response()
}

async fn ready(State(state): State<AppState>) -> Response {
    match state.db.health_check().await {
        Ok(_) => (StatusCode::OK, "Ready").into_response(),
        Err(e) => {
            tracing::error!("Readiness check failed: {}", e);
            (StatusCode::SERVICE_UNAVAILABLE, "Not Ready").into_response()
        }
    }
}

async fn get_metrics(State(state): State<AppState>) -> Response {
    match state.metrics.gather() {
        Ok(metrics) => (
            StatusCode::OK,
            [("Content-Type", "text/plain; version=0.0.4")],
            metrics,
        )
            .into_response(),
        Err(e) => {
            tracing::error!("Failed to gather metrics: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to gather metrics").into_response()
        }
    }
}

async fn status(State(state): State<AppState>) -> Response {
    let db_status = match state.db.health_check().await {
        Ok(_) => "healthy",
        Err(_) => "unhealthy",
    };

    let response = serde_json::json!({
        "status": "operational",
        "database": db_status,
        "version": env!("CARGO_PKG_VERSION")
    });

    (
        StatusCode::OK,
        [("Content-Type", "application/json")],
        serde_json::to_string(&response).unwrap(),
    )
        .into_response()
}

