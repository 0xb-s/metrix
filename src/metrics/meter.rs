// src/metrics/meter.rs

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;

use super::Metric;

/// A meter metric to track rates.
pub struct Meter {
    name: String,
    pub labels: HashMap<String, String>,
    count: Mutex<u64>,
    start_time: Instant,
}

impl Meter {
    /// Creates a new meter.
    pub fn new(name: &str, labels: HashMap<String, String>) -> Self {
        Meter {
            name: name.to_string(),
            labels,
            count: Mutex::new(0),
            start_time: Instant::now(),
        }
    }

    /// Marks an event occurrence.
    pub fn mark(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
    }

    /// Gets the rate of events per second.
    pub fn get_rate(&self) -> f64 {
        let count = self.count.lock().unwrap();
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed > 0.0 {
            *count as f64 / elapsed
        } else {
            0.0
        }
    }
}

impl Metric for Meter {
    fn name(&self) -> &str {
        &self.name
    }

    fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}
