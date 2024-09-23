// src/metrics/histogram.rs

use std::collections::HashMap;
use std::sync::Mutex;

use super::Metric;

/// A histogram metric.
pub struct Histogram {
    name: String,
    labels: HashMap<String, String>,
    observations: Mutex<Vec<f64>>,
}

impl Histogram {
    /// Creates a new histogram.
    pub fn new(name: &str, labels: HashMap<String, String>) -> Self {
        Histogram {
            name: name.to_string(),
            labels,
            observations: Mutex::new(Vec::new()),
        }
    }

    /// Records an observation.
    pub fn observe(&self, value: f64) {
        let mut obs = self.observations.lock().unwrap();
        obs.push(value);
    }

    /// Gets the percentile value.
    pub fn get_percentile(&self, percentile: f64) -> Option<f64> {
        let mut obs = self.observations.lock().unwrap();
        if obs.is_empty() {
            return None;
        }
        obs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let index = (percentile / 100.0 * obs.len() as f64).ceil() as usize - 1;
        obs.get(index).cloned()
    }

    /// Gets a summary of the histogram.
    pub fn get_summary(&self) -> HistogramSummary {
        let obs = self.observations.lock().unwrap();
        HistogramSummary::from_observations(&obs)
    }
}

impl Metric for Histogram {
    fn name(&self) -> &str {
        &self.name
    }

    fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}

/// A summary of histogram data.
pub struct HistogramSummary {
    pub count: usize,
    pub sum: f64,
    pub min: f64,
    pub max: f64,
    pub mean: f64,
    pub std_dev: f64,
}

impl HistogramSummary {
    /// Creates a summary from observations.
    pub fn from_observations(observations: &[f64]) -> Self {
        let count = observations.len();
        let sum: f64 = observations.iter().sum();
        let min = observations.iter().cloned().fold(f64::INFINITY, f64::min);
        let max = observations
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let mean = sum / count as f64;
        let variance = observations.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / count as f64;
        let std_dev = variance.sqrt();
        HistogramSummary {
            count,
            sum,
            min,
            max,
            mean,
            std_dev,
        }
    }
}
