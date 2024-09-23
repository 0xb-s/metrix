use crate::metrics::Metric;
use crate::registry::Registry;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::serve;
use axum::{routing::get, Router};
use serde_json::json;
use std::sync::Arc;
use tokio::net::TcpListener;
pub async fn start_json_exporter(
    registry: Arc<Registry>,
    addr: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/metrics.json", get(metrics_handler))
        .with_state(registry);

    let listener = TcpListener::bind(addr).await?;
    println!("JSON exporter listening on http://{}", addr);

    serve(listener, app).await?;

    Ok(())
}

async fn metrics_handler(State(registry): State<Arc<Registry>>) -> impl IntoResponse {
    let metrics = collect_metrics_json(&registry);
    (
        axum::http::StatusCode::OK,
        [("Content-Type", "application/json")],
        metrics,
    )
}

fn collect_metrics_json(registry: &Arc<Registry>) -> String {
    let mut metrics = serde_json::Map::new();

    // Collect counters
    let counters = registry.counters.read().unwrap();
    for counter in counters.values() {
        let name = counter.name();
        let value = counter.get();
        metrics.insert(name.to_string(), json!(value));
    }

    // Collect gauges
    let gauges = registry.gauges.read().unwrap();
    for gauge in gauges.values() {
        let name = gauge.name();
        let value = gauge.get();
        metrics.insert(name.to_string(), json!(value));
    }

    // TODO other:

    serde_json::to_string(&metrics).unwrap()
}
