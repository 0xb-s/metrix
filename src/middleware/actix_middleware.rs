use crate::registry::Registry;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::Error;
use futures::future::{ok, LocalBoxFuture, Ready};
use futures::task::Context;
use futures::task::Poll;
use std::collections::HashMap;
use std::sync::Arc;
pub struct MetricsMiddleware {
    registry: Arc<Registry>,
}

impl MetricsMiddleware {
    pub fn new(registry: Arc<Registry>) -> Self {
        MetricsMiddleware { registry }
    }
}

impl<S, B> Transform<S, ServiceRequest> for MetricsMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = MetricsMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MetricsMiddlewareService {
            service,
            registry: self.registry.clone(),
        })
    }
}

pub struct MetricsMiddlewareService<S> {
    service: S,
    registry: Arc<Registry>,
}

impl<S, B> Service<ServiceRequest> for MetricsMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let registry = self.registry.clone();
        let method = req.method().to_string();
        let path = req.path().to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            // Record metrics
            let mut labels = HashMap::new();
            labels.insert("method".to_string(), method);
            labels.insert("path".to_string(), path);

            let counter = registry.register_counter("http_requests_total", labels);
            counter.increment();

            Ok(res)
        })
    }
}
