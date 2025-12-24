use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use std::time::Instant;

use crate::metrics::Metrics;

pub async fn metrics_middleware(
    State(metrics): State<Arc<Metrics>>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let start = Instant::now();
    let method = request.method().clone();
    let uri = request.uri().clone();

    // Extract endpoint path
    let endpoint = normalize_path(uri.path());

    // Increment active connections
    metrics.active_connections.inc();

    // Process request
    let response = next.run(request).await;

    // Calculate duration
    let duration = start.elapsed().as_secs_f64();

    // Get status code
    let status = response.status().as_u16();

    // Record metrics
    metrics
        .http_requests_total
        .with_label_values(&[
            method.as_str(),
            &endpoint,
            &status.to_string(),
        ])
        .inc();

    metrics
        .http_request_duration_seconds
        .with_label_values(&[method.as_str(), &endpoint])
        .observe(duration);

    // Decrement active connections
    metrics.active_connections.dec();

    response
}

fn normalize_path(path: &str) -> String {
    // Normalize paths to avoid high cardinality
    // For example: /api/v1/users/123 -> /api/v1/users/:id
    if path.starts_with("/api/") {
        // For API routes, try to normalize IDs
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() > 3 {
            // Check if last part looks like an ID (numeric or UUID-like)
            let last = parts.last().unwrap();
            if last.parse::<u64>().is_ok() || last.len() > 10 {
                // Replace with placeholder
                let mut normalized = parts[..parts.len() - 1].join("/");
                normalized.push_str("/:id");
                return normalized;
            }
        }
    }
    path.to_string()
}

