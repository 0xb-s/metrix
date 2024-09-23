// src/metrics/mod.rs

pub mod async_timer;
pub mod counter;
pub mod gauge;
pub mod histogram;
pub mod meter;
pub mod timer;

pub use counter::Counter;
pub use gauge::Gauge;
pub use histogram::Histogram;
pub use meter::Meter;
pub use timer::Timer;

/// Trait representing a metric.
pub trait Metric {
    fn name(&self) -> &str;
    fn labels(&self) -> &std::collections::HashMap<String, String>;
}
