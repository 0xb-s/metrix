// src/macros.rs

/// Increments a counter metric.
///
/// # Examples
///
/// ```
/// metrics_counter!(registry, "requests_total", labels);
/// ```
#[macro_export]
macro_rules! metrics_counter {
    ($registry:expr, $name:expr, $labels:expr) => {{
        let counter = $registry.register_counter($name, $labels);
        counter.increment();
    }};
    ($registry:expr, $name:expr, $labels:expr, $amount:expr) => {{
        let counter = $registry.register_counter($name, $labels);
        counter.increment_by($amount);
    }};
}

/// Sets a gauge metric.
///
/// # Examples
///
/// ```
/// metrics_gauge!(registry, "memory_usage", labels, value);
/// ```
#[macro_export]
macro_rules! metrics_gauge {
    ($registry:expr, $name:expr, $labels:expr, $value:expr) => {{
        let gauge = $registry.register_gauge($name, $labels);
        gauge.set($value);
    }};
}

/// Measures the duration of a code block.
///
/// # Examples
///
/// ```
/// metrics_timer!(registry, "request_duration_seconds", labels, {
///     // Code to measure
/// });
/// ```
#[macro_export]
macro_rules! metrics_timer {
    ($registry:expr, $name:expr, $labels:expr, $code:block) => {{
        let timer = $registry.register_timer($name, $labels);
        let handle = timer.start();
        let result = { $code };
        handle.stop();
        result
    }};
}
