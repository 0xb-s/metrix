use crate::metrics::Metric;
use crate::registry::Registry;
use axum::{extract::State, response::IntoResponse, routing::get, serve, Router};
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct PrometheusExporter {
    registry: Arc<Registry>,
}

impl PrometheusExporter {
    pub fn new(registry: Arc<Registry>) -> Self {
        PrometheusExporter { registry }
    }

    pub async fn start(self, addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let registry = Arc::clone(&self.registry);

        let app = Router::new()
            .route("/metrics", get(metrics_handler))
            .with_state(registry);

        let listener = TcpListener::bind(addr).await?;
        println!("Prometheus exporter listening on http://{}", addr);

        serve(listener, app).await?;

        Ok(())
    }
}

async fn metrics_handler(State(registry): State<Arc<Registry>>) -> impl IntoResponse {
    let metrics = collect_metrics(&registry);
    (
        axum::http::StatusCode::OK,
        [("Content-Type", "text/plain; version=0.0.4")],
        metrics,
    )
}

fn collect_metrics(registry: &Arc<Registry>) -> String {
    let mut output = String::new();

    // Collect counters
    let counters = registry.counters.read().unwrap();
    for counter in counters.values() {
        let name = sanitize_metric_name(&counter.name());
        let value = counter.get();
        let labels = format_labels(&counter.labels());
        output.push_str(&format!("{}{} {}\n", name, labels, value));
    }

    // Collect gauges
    let gauges = registry.gauges.read().unwrap();
    for gauge in gauges.values() {
        let name = sanitize_metric_name(&gauge.name());
        let value = gauge.get();
        let labels = format_labels(&gauge.labels());
        output.push_str(&format!("{}{} {}\n", name, labels, value));
    }

    // Histograms, meters, timers can be added similarly.

    output
}

fn format_labels(labels: &std::collections::HashMap<String, String>) -> String {
    if labels.is_empty() {
        "".to_string()
    } else {
        let label_pairs: Vec<String> = labels
            .iter()
            .map(|(k, v)| format!("{}=\"{}\"", k, v))
            .collect();
        format!("{{{}}}", label_pairs.join(","))
    }
}

fn sanitize_metric_name(name: &str) -> String {
    name.replace('.', "_").replace('-', "_")
}
