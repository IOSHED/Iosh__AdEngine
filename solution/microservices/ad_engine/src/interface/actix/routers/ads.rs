use crate::{domain, infrastructure, interface};


#[derive(serde::Deserialize, Debug)]
struct AdsQuery {
    client_id: uuid::Uuid,
}

#[utoipa::path(
    get,
    path = "/ads",
    tag = "ads",
    params(
        ("client_id" = uuid::Uuid, Query, description = "Id client for getting ads", example = "3fa85f64-5717-4562-b3fc-2c963f66afa6"),
    ),
    responses(
        (status = 200, description = "List campaigns", body = Vec<domain::schemas::CampaignSchema>),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::get("/ads")]
#[tracing::instrument(name = "Get list of campaigns", skip(db_pool))]
pub async fn ads_handler(
    ads_query: actix_web::web::Query<AdsQuery>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
    redis_pool: actix_web::web::Data<infrastructure::database_connection::redis::RedisPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let pagination = ads_query.into_inner();

    let ads = domain::usecase::AdsGetUsecase::new(db_pool.get_ref(), redis_pool.get_ref())
        .execute(pagination.client_id)
        .await?;

    Ok(actix_web::HttpResponse::Created().json(ads))
}
