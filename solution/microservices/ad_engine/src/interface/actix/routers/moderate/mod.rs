use crate::{domain, infrastructure, interface};

pub fn moderate_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path).service(moderate_config_handler)
}

#[utoipa::path(
    post,
    path = "/moderate/config",
    tag = "Moderate",
    request_body = domain::schemas::ModerateSchema,
    responses(
        (status = 200, description = "Set activate moderate to value", body = domain::schemas::ModerateSchema),
        (status = 400, description = "Bad activate moderate", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::post("/config")]
pub async fn moderate_config_handler(
    modarate_request: actix_web::web::Json<domain::schemas::ModerateSchema>,
    redis_pool: actix_web::web::Data<infrastructure::database_connection::redis::RedisPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let response = domain::usecase::ModerateSetSettingsUsecase::new(redis_pool.get_ref())
        .set(modarate_request.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::Ok().json(response))
}
