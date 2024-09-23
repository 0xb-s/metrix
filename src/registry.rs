// src/registry.rs

use crate::metrics::{Counter, Gauge, Histogram, Meter, Timer};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// A registry to manage all metrics.
pub struct Registry {
    pub counters: RwLock<HashMap<String, Arc<Counter>>>,
    pub gauges: RwLock<HashMap<String, Arc<Gauge>>>,
    histograms: RwLock<HashMap<String, Arc<Histogram>>>,
    meters: RwLock<HashMap<String, Arc<Meter>>>,
    timers: RwLock<HashMap<String, Arc<Timer>>>,
}

impl Registry {
    /// Creates a new registry.
    pub fn new() -> Self {
        Registry {
            counters: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
            histograms: RwLock::new(HashMap::new()),
            meters: RwLock::new(HashMap::new()),
            timers: RwLock::new(HashMap::new()),
        }
    }

    /// Registers or retrieves a counter.
    pub fn register_counter(&self, name: &str, labels: HashMap<String, String>) -> Arc<Counter> {
        let mut counters = self.counters.write().unwrap();
        counters
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Counter::new(name, labels)))
            .clone()
    }

    /// Registers or retrieves a gauge.
    pub fn register_gauge(&self, name: &str, labels: HashMap<String, String>) -> Arc<Gauge> {
        let mut gauges = self.gauges.write().unwrap();
        gauges
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Gauge::new(name, labels)))
            .clone()
    }

    /// Registers or retrieves a histogram.
    pub fn register_histogram(
        &self,
        name: &str,
        labels: HashMap<String, String>,
    ) -> Arc<Histogram> {
        let mut histograms = self.histograms.write().unwrap();
        histograms
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Histogram::new(name, labels)))
            .clone()
    }

    /// Registers or retrieves a meter.
    pub fn register_meter(&self, name: &str, labels: HashMap<String, String>) -> Arc<Meter> {
        let mut meters = self.meters.write().unwrap();
        meters
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Meter::new(name, labels)))
            .clone()
    }

    /// Registers or retrieves a timer.
    pub fn register_timer(&self, name: &str, labels: HashMap<String, String>) -> Arc<Timer> {
        let mut timers = self.timers.write().unwrap();
        timers
            .entry(name.to_string())
            .or_insert_with(|| Arc::new(Timer::new(name, labels)))
            .clone()
    }

    // Methods to collect and export metrics can be added here.
}
