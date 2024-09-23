use actix_web::{web, App, HttpServer, Responder};
use metrix::middleware::actix_middleware::MetricsMiddleware;
use metrix::registry::Registry;
use std::sync::Arc;

async fn index() -> impl Responder {
    "Hello, World!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let registry = Arc::new(Registry::new());

    // Start Prometheus exporter
    let exporter_registry = Arc::clone(&registry);
    tokio::spawn(async move {
        metrix::exporters::prometheus::PrometheusExporter::new(exporter_registry)
            .start("127.0.0.1:9100")
            .await
            .unwrap();
    });

    HttpServer::new(move || {
        App::new()
            .wrap(MetricsMiddleware::new(registry.clone()))
            .route("/", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
