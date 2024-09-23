// src/metrics/counter.rs

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

use super::Metric;

/// A counter metric.
pub struct Counter {
    name: String,
    labels: HashMap<String, String>,
    value: AtomicU64,
}

impl Counter {
    /// Creates a new counter.
    pub fn new(name: &str, labels: HashMap<String, String>) -> Self {
        Counter {
            name: name.to_string(),
            labels,
            value: AtomicU64::new(0),
        }
    }

    /// Increments the counter by 1.
    pub fn increment(&self) {
        self.value.fetch_add(1, Ordering::Relaxed);
    }

    /// Increments the counter by a specified amount.
    pub fn increment_by(&self, amount: u64) {
        self.value.fetch_add(amount, Ordering::Relaxed);
    }

    /// Gets the current value of the counter.
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
}

impl Metric for Counter {
    fn name(&self) -> &str {
        &self.name
    }

    fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}
