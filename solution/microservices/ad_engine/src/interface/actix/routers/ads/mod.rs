use crate::{domain, infrastructure, interface};

pub fn ads_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .service(ads_handler)
        .service(ads_click_handler)
}

#[derive(serde::Deserialize, Debug)]
struct AdsQuery {
    client_id: uuid::Uuid,
}

#[utoipa::path(
    get,
    path = "/ads",
    tag = "Ads", 
    params(
        ("client_id" = uuid::Uuid, Query, description = "Id client for getting ads", example = "3fa85f64-5717-4562-b3fc-2c963f66afa6"),
    ),
    responses(
        (status = 200, description = "List campaigns", body = domain::schemas::AdSchema),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 404, description = "There is no suitable advertisement", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::get("")]
#[tracing::instrument(
    name = "ads_handler",
    skip(db_pool, redis_pool, app_state),
    fields(
        client_id = %ads_query.client_id,
        request_id = %uuid::Uuid::new_v4()
    )
)]
pub async fn ads_handler(
    ads_query: actix_web::web::Query<AdsQuery>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
    redis_pool: actix_web::web::Data<infrastructure::database_connection::redis::RedisPool>,
    app_state: actix_web::web::Data<domain::configurate::AppState>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let pagination = ads_query.into_inner();

    let ads = domain::usecase::AdsGetUsecase::new(db_pool.get_ref(), redis_pool.get_ref(), app_state.get_ref())
        .execute(pagination.client_id)
        .await?;

    Ok(actix_web::HttpResponse::Created().json(ads))
}

#[utoipa::path(
    post,
    path = "/ads/{ads_id}/click",
    tag = "Ads",
    request_body = domain::schemas::AdClickRequest,
    responses(
        (status = 204, description = "Click successful", body = ()),
        (status = 400, description = "Bot found this campaign", body = interface::actix::exception::ExceptionResponse),
        (status = 404, description = "Not found this client", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::post("/{ads_id}/click")]
#[tracing::instrument(
    name = "ads_click_handler",
    skip(db_pool, redis_pool, ads_request),
    fields(
        campaign_id = %campaign_id,
        client_id = %ads_request.client_id,
        request_id = %uuid::Uuid::new_v4()
    )
)]
pub async fn ads_click_handler(
    campaign_id: actix_web::web::Path<uuid::Uuid>,
    ads_request: actix_web::web::Json<domain::schemas::AdClickRequest>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
    redis_pool: actix_web::web::Data<infrastructure::database_connection::redis::RedisPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    domain::usecase::AdsClickUsecase::new(db_pool.get_ref(), redis_pool.get_ref())
        .click(campaign_id.into_inner(), ads_request.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::NoContent().into())
}
