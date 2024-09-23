use crate::metrics::{Histogram, Meter};
use futures::Future;
use std::sync::Arc;
use tokio::time::Instant;

pub struct AsyncTimer {
    histogram: Arc<Histogram>,
    meter: Arc<Meter>,
}

impl AsyncTimer {
    pub fn new(histogram: Arc<Histogram>, meter: Arc<Meter>) -> Self {
        AsyncTimer { histogram, meter }
    }

    pub async fn time<F, Fut, R>(&self, f: F) -> R
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = R>,
    {
        let start = Instant::now();
        let result = f().await;
        let duration = start.elapsed();
        self.histogram.observe(duration.as_secs_f64());
        self.meter.mark();
        result
    }
}
