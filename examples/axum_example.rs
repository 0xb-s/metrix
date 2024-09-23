// examples/axum_example.rs

use axum::{
    extract::State,
    http::Request,
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    serve, Router,
};
use metrix::{exporters::prometheus::PrometheusExporter, registry::Registry};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::task;

#[tokio::main]
async fn main() {
    let registry = Arc::new(Registry::new());

    // Start Prometheus exporter in a separate task
    let exporter_registry = Arc::clone(&registry);
    task::spawn(async move {
        PrometheusExporter::new(exporter_registry)
            .start("127.0.0.1:9100")
            .await
            .unwrap();
    });

    // Build the Axum application
    let app = Router::new()
        .route("/", get(root_handler))
        .layer(middleware::from_fn_with_state(
            registry.clone(),
            metrics_middleware,
        ))
        .with_state(registry.clone());

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running at http://127.0.0.1:3000");

    serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    "Hello, World!"
}

use axum::body::Body;

async fn metrics_middleware(
    State(registry): State<Arc<Registry>>,
    req: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    // Proceed to the next handler
    let response = next.run(req).await;

    // Record metrics using your Registry
    let mut labels = std::collections::HashMap::new();
    labels.insert("method".to_string(), method);
    labels.insert("path".to_string(), path);

    // Increment request counter
    let counter = registry.register_counter("http_requests_total", labels);
    counter.increment();

    response
}
