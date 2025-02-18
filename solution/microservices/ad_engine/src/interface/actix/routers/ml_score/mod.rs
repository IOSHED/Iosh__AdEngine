use crate::{domain, infrastructure, interface};

#[utoipa::path(
    post,
    path = "/ml-score",
    tag = "ML",
    request_body = domain::schemas::MlScoreRequest,
    responses(
        (status = 200, description = "Score calculation successful", body = ()),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 404, description = "Not found advertiser or client with id",
            body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[tracing::instrument(name = "ml_score_handler", skip(db_pool))]
#[actix_web::post("/ml-score")]
pub async fn ml_score_handler(
    score_request: actix_web::web::Json<domain::schemas::MlScoreRequest>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let _score_result = domain::usecase::MlScoreUsecase::new(db_pool.get_ref())
        .set_ml_score(score_request.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::Ok().into())
}
