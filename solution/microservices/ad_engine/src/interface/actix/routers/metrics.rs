use crate::domain;

#[actix_web::get("/metrics")]
async fn metrics_handler() -> actix_web::HttpResponse {
    actix_web::HttpResponse::Ok()
        .content_type("text/plain; version=0.0.4")
        .body(domain::services::PrometheusService::get_metrics().await)
}
