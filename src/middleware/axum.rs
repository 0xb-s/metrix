use axum::body::Body;
use axum::extract::Extension;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use std::collections::HashMap;
use std::sync::Arc;

use crate::registry::Registry;

/// Axum middleware for metrics.
pub async fn metrics_middleware(
    req: Request<Body>,
    next: Next,
    Extension(registry): Extension<Arc<Registry>>,
) -> Response {
    let path = req.uri().path().to_string();

    let labels = {
        let mut labels = HashMap::new();
        labels.insert("path".to_string(), path.clone());
        labels
    };

    // Start timer
    let timer = registry.register_timer("http_request_duration_seconds", labels.clone());
    let handle = timer.start();

    // Increment request counter
    let counter = registry.register_counter("http_requests_total", labels.clone());
    counter.increment();

    // Proceed to the next middleware or handler
    let response = next.run(req).await;

    // Stop timer
    handle.stop();

    response
}
