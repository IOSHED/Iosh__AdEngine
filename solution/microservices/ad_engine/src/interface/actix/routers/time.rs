use crate::{domain, infrastructure, interface};

#[utoipa::path(
    post,
    path = "/time/advance",
    tag = "Time",
    responses(
        (status = 200, description = "Set time to value", body = domain::schemas::TimeAdvanceResponse),
        (status = 400, description = "Bad time", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::post("/time/advance")]
#[tracing::instrument(name = "time_advance_handler", skip(redis_pool, db_pool))]
pub async fn time_advance_handler(
    time_advance_request: actix_web::web::Json<domain::schemas::TimeAdvanceRequest>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
    redis_pool: actix_web::web::Data<infrastructure::database_connection::redis::RedisPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let response = domain::usecase::TimeAdvanceUsecase::new(db_pool.get_ref(), redis_pool.get_ref())
        .set_advance(time_advance_request.into_inner())
        .await?;
    Ok(actix_web::HttpResponse::Ok().json(response))
}
