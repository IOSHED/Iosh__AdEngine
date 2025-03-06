use std::future::Future;

use actix_web::dev::Service;

use crate::domain;

pub struct MetricsMiddlewareService<S> {
    service: S,
}

impl<S> Service<actix_web::dev::ServiceRequest> for MetricsMiddlewareService<S>
where
    S: Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse, Error = actix_web::Error>
        + 'static,
{
    type Error = actix_web::Error;
    type Future = std::pin::Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    type Response = actix_web::dev::ServiceResponse;

    fn poll_ready(&self, ctx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        let start_time = actix_web::rt::time::Instant::now();

        let request_path = req.path().to_string();
        let is_registered_resource = req.resource_map().has_resource(request_path.as_str());
        let request_method = req.method().to_string();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            let elapsed = start_time.elapsed().as_secs_f64();

            if is_registered_resource {
                domain::services::PrometheusService::increment_http_requests_total(&request_method, &request_path);
                domain::services::PrometheusService::observe_http_response_time(
                    elapsed,
                    &request_method,
                    &request_path,
                );
            }

            Ok(res)
        })
    }
}

pub struct MetricsMiddleware;

impl<S> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for MetricsMiddleware
where
    S: actix_web::dev::Service<
            actix_web::dev::ServiceRequest,
            Response = actix_web::dev::ServiceResponse,
            Error = actix_web::Error,
        > + 'static,
{
    type Error = actix_web::Error;
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;
    type InitError = ();
    type Response = S::Response;
    type Transform = MetricsMiddlewareService<S>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ready(Ok(MetricsMiddlewareService { service }))
    }
}
