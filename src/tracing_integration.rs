use crate::registry::Registry;
use std::collections::HashMap;
use std::sync::Arc;
use tracing::Subscriber;
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::registry::LookupSpan;
pub struct MetricsLayer {
    registry: Arc<Registry>,
}

impl MetricsLayer {
    pub fn new(registry: Arc<Registry>) -> Self {
        MetricsLayer { registry }
    }
}

impl<S> Layer<S> for MetricsLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_enter(&self, id: &tracing::Id, ctx: Context<S>) {
        if let Some(span) = ctx.span(id) {
            let name = span.name().to_string();
            let labels = HashMap::new();
            let counter = self
                .registry
                .register_counter(&format!("{}_entered", name), labels);
            counter.increment();
        }
    }

    fn on_exit(&self, id: &tracing::Id, ctx: Context<S>) {
        if let Some(span) = ctx.span(id) {
            let name = span.name().to_string();
            let labels = HashMap::new();
            let counter = self
                .registry
                .register_counter(&format!("{}_exited", name), labels);
            counter.increment();
        }
    }
}
