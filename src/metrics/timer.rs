

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::Metric;

/// A timer metric to measure durations.
pub struct Timer {
    name: String,
    labels: HashMap<String, String>,
    observations: Mutex<Vec<Duration>>,
}

impl Timer {
    /// Creates a new timer.
    pub fn new(name: &str, labels: HashMap<String, String>) -> Self {
        Timer {
            name: name.to_string(),
            labels,
            observations: Mutex::new(Vec::new()),
        }
    }

    /// Starts a timing operation.
    pub fn start(&self) -> TimerHandle {
        TimerHandle {
            start_time: Instant::now(),
            timer: self,
        }
    }

    /// Observes a duration.
    pub fn observe_duration(&self, duration: Duration) {
        let mut obs = self.observations.lock().unwrap();
        obs.push(duration);
    }

    /// Gets a summary of the timer data.
    pub fn get_summary(&self) -> TimerSummary {
        let obs = self.observations.lock().unwrap();
        TimerSummary::from_observations(&obs)
    }
}

impl Metric for Timer {
    fn name(&self) -> &str {
        &self.name
    }

    fn labels(&self) -> &HashMap<String, String> {
        &self.labels
    }
}

/// A handle to a timing operation.
pub struct TimerHandle<'a> {
    start_time: Instant,
    timer: &'a Timer,
}

impl<'a> TimerHandle<'a> {
    /// Stops the timing operation and records the duration.
    pub fn stop(self) {
        let duration = self.start_time.elapsed();
        self.timer.observe_duration(duration);
    }
}

/// A summary of timer data.
pub struct TimerSummary {
    pub count: usize,
    pub sum: Duration,
    pub min: Duration,
    pub max: Duration,
    pub mean: Duration,
    pub std_dev: Duration,
}

impl TimerSummary {
    /// Creates a summary from observations.
    pub fn from_observations(observations: &[Duration]) -> Self {
        let count = observations.len();
        let sum: Duration = observations.iter().sum();
        let min = observations.iter().cloned().min().unwrap_or_default();
        let max = observations.iter().cloned().max().unwrap_or_default();
        let mean = if count > 0 {
            sum / count as u32
        } else {
            Duration::default()
        };

        let variance = observations
            .iter()
            .map(|&x| {
                let diff = x.as_secs_f64() - mean.as_secs_f64();
                diff * diff
            })
            .sum::<f64>()
            / count as f64;
        let std_dev_secs = variance.sqrt();
        let std_dev = Duration::from_secs_f64(std_dev_secs);

        TimerSummary {
            count,
            sum,
            min,
            max,
            mean,
            std_dev,
        }
    }
}
