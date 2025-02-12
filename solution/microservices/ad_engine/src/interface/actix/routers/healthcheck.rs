#[utoipa::path(
    get,
    path = "/ping",
    tag = "Ping",
    responses(
        (status = 200, description = "Check app on healthy", example = "pong")
    )
)]
#[actix_web::get("/ping")]
#[tracing::instrument(name = "healthcheck_handler get ping")]
pub async fn healthcheck_handler() -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body("pong")
}
