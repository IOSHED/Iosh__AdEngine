use crate::{domain, infrastructure, interface};

pub fn campaigns_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .service(campaigns_create_handler)
        .service(campaigns_delete_handler)
        .service(campaigns_update_handler)
        .service(campaigns_get_by_id_handler)
        .service(campaigns_get_list_handler)
}

#[utoipa::path(
    post,
    path = "/advertisers/{advertiser_id}/campaigns",
    tag = "campaigns",
    request_body = domain::schemas::CampaignsCreateRequest,
    responses(
        (status = 201, description = "Created campaign", body = domain::schemas::CampaignSchema),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::post("")]
#[tracing::instrument(name = "Create campaign", skip(db_pool))]
pub async fn campaigns_create_handler(
    campaign_request: actix_web::web::Json<domain::schemas::CampaignsCreateRequest>,
    advertiser_id: actix_web::web::Path<uuid::Uuid>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
    redis_pool: actix_web::web::Data<infrastructure::database_connection::redis::RedisPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let campaign = domain::usecase::CampaignsCreateUsecase::new(db_pool.get_ref(), redis_pool.get_ref())
        .create(campaign_request.into_inner(), advertiser_id.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::Created().json(campaign))
}

#[utoipa::path(
    put,
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}",
    tag = "campaigns",
    request_body = domain::schemas::CampaignsUpdateRequest,
    responses(
        (status = 200, description = "Updated campaign", body = domain::schemas::CampaignSchema),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::put("/{campaign_id}")]
#[tracing::instrument(name = "Update campaign", skip(db_pool))]
pub async fn campaigns_update_handler(
    campaign_request: actix_web::web::Json<domain::schemas::CampaignsUpdateRequest>,
    path_param: actix_web::web::Path<(uuid::Uuid, uuid::Uuid)>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
    redis_pool: actix_web::web::Data<infrastructure::database_connection::redis::RedisPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let (advertiser_id, campaign_id) = path_param.into_inner();
    let campaign = domain::usecase::CampaignsUpdateUsecase::new(db_pool.get_ref(), redis_pool.get_ref())
        .update(campaign_request.into_inner(), advertiser_id, campaign_id)
        .await?;

    Ok(actix_web::HttpResponse::Ok().json(campaign))
}

#[utoipa::path(
    delete,
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}",
    tag = "campaigns", 
    responses(
        (status = 204, description = "Deleted", body = ()),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::delete("/{campaign_id}")]
#[tracing::instrument(name = "Delete campaign", skip(db_pool))]
pub async fn campaigns_delete_handler(
    path_param: actix_web::web::Path<(uuid::Uuid, uuid::Uuid)>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
    redis_pool: actix_web::web::Data<infrastructure::database_connection::redis::RedisPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let (advertiser_id, campaign_id) = path_param.into_inner();
    domain::usecase::CampaignsDeleteUsecase::new(db_pool.get_ref(), redis_pool.get_ref())
        .delete(advertiser_id, campaign_id)
        .await?;

    Ok(actix_web::HttpResponse::NoContent().finish())
}

#[utoipa::path(
    get,
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}",
    tag = "campaigns",
    responses(
        (status = 200, description = "Got campaign", body = domain::schemas::CampaignSchema),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::get("/{campaign_id}")]
#[tracing::instrument(name = "Get campaign by id", skip(db_pool))]
pub async fn campaigns_get_by_id_handler(
    path_param: actix_web::web::Path<(uuid::Uuid, uuid::Uuid)>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let (advertiser_id, campaign_id) = path_param.into_inner();
    let campaign = domain::usecase::CampaignsGetByIdUsecase::new(db_pool.get_ref())
        .get(advertiser_id, campaign_id)
        .await?;

    Ok(actix_web::HttpResponse::Created().json(campaign))
}

#[derive(serde::Deserialize, Debug)]
struct Pagination {
    size: Option<u32>,
    page: Option<u32>,
}

#[utoipa::path(
    get,
    path = "/advertisers/{advertiser_id}/campaigns",
    tag = "campaigns",
    params(
        ("size" = Option<u32>, Query, description = "Number of items per page", example = 10),
        ("page" = Option<u32>, Query, description = "Page number", example = 1),
    ),
    responses(
        (status = 200, description = "List campaigns", body = Vec<domain::schemas::CampaignSchema>),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::get("")]
#[tracing::instrument(name = "Get list of campaigns", skip(db_pool))]
pub async fn campaigns_get_list_handler(
    advertiser_id: actix_web::web::Path<uuid::Uuid>,
    pagination: actix_web::web::Query<Pagination>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let pagination = pagination.into_inner();
    let size = pagination.size.unwrap_or(10);
    let page = pagination.page.unwrap_or(1);
    let (total_count, campaigns) = domain::usecase::CampaignsGetListUsecase::new(db_pool.get_ref())
        .get(advertiser_id.into_inner(), size, page)
        .await?;

    Ok(actix_web::HttpResponse::Created()
        .append_header(("x-total-count", total_count.to_string()))
        .json(campaigns))
}
