pub fn exponential_buckets(start: f64, factor: f64, count: usize) -> Vec<f64> {
    let mut buckets = Vec::with_capacity(count);
    let mut current = start;
    for _ in 0..count {
        buckets.push(current);
        current *= factor;
    }
    buckets
}

pub fn linear_buckets(start: f64, width: f64, count: usize) -> Vec<f64> {
    (0..count).map(|i| start + (i as f64) * width).collect()
}
