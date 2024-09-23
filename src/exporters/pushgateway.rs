use crate::registry::Registry;
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;

pub struct PushgatewayExporter {
    registry: Arc<Registry>,
    client: Client,
    push_url: String,
    job: String,
    interval: Duration,
}

impl PushgatewayExporter {
    pub fn new(registry: Arc<Registry>, push_url: String, job: String, interval: Duration) -> Self {
        PushgatewayExporter {
            registry,
            client: Client::new(),
            push_url,
            job,
            interval,
        }
    }

    pub async fn start(self) {
        let mut interval = interval(self.interval);

        loop {
            interval.tick().await;
            if let Err(e) = self.push_metrics().await {
                eprintln!("Error pushing metrics: {}", e);
            }
        }
    }

    async fn push_metrics(&self) -> Result<(), Box<dyn std::error::Error>> {
        let metrics = self.collect_metrics();

        let url = format!("{}/metrics/job/{}", self.push_url, self.job);

        let response = self.client.post(&url).body(metrics).send().await?;

        if !response.status().is_success() {
            eprintln!("Failed to push metrics: {}", response.status());
        }

        Ok(())
    }

    fn collect_metrics(&self) -> String {
        crate::exporters::prometheus::collect_metrics(&self.registry)
    }
}
