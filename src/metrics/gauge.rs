// src/metrics/gauge.rs

use std::collections::HashMap;
use std::sync::Mutex;

use super::Metric;

/// A gauge metric.
pub struct Gauge {
    pub name: String,
    pub labels: HashMap<String, String>,
    pub value: Mutex<f64>,
}

impl Gauge {
    /// Creates a new gauge.
    pub fn new(name: &str, labels: HashMap<String, String>) -> Self {
        Gauge {
            name: name.to_string(),
            labels,
            value: Mutex::new(0.0),
        }
    }

    /// Sets the gauge to a specific value.
    pub fn set(&self, value: f64) {
        let mut val = self.value.lock().unwrap();
        *val = value;
    }

    /// Increments the gauge by 1.
    pub fn increment(&self) {
        let mut val = self.value.lock().unwrap();
        *val += 1.0;
    }

    /// Decrements the gauge by 1.
    pub fn decrement(&self) {
        let mut val = self.value.lock().unwrap();
        *val -= 1.0;
    }

    /// Gets the current value of the gauge.
    pub fn get(&self) -> f64 {
        let val = self.value.lock().unwrap();
        *val
    }
}

impl Metric for Gauge {
    fn name(&self) -> &str {
        &self.name
    }

    fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}
