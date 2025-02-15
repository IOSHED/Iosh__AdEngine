use crate::{domain, infrastructure, interface};

pub fn stat_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .service(stat_campaign_daily_handler)
        .service(stat_campaign_handler)
        .service(stat_advertisers_handler)
        .service(stat_advertisers_daily_handler)
}

#[utoipa::path(
    get,
    path = "/stats/campaigns/{campaign_id}",
    tag = "Stats",
    responses(
        (status = 200, description = "Got stat", body = domain::schemas::StatResponse),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::get("/campaigns/{campaign_id}")]
#[tracing::instrument(name = "Get stat campaign by id", skip(db_pool))]
pub async fn stat_campaign_handler(
    campaign_id: actix_web::web::Path<uuid::Uuid>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let stat = domain::usecase::StatCampaignUsecase::new(db_pool.get_ref())
        .get(campaign_id.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::Ok().json(stat))
}

#[utoipa::path(
    get,
    path = "/stats/campaigns/{campaign_id}/daily",
    tag = "Stats",
    responses(
        (status = 200, description = "Got stat", body = Vec<domain::schemas::StatDailyResponse>),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::get("/campaigns/{campaign_id}/daily")]
#[tracing::instrument(name = "Get stat campaign by id", skip(db_pool))]
pub async fn stat_campaign_daily_handler(
    campaign_id: actix_web::web::Path<uuid::Uuid>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let stat = domain::usecase::StatCampaignUsecase::new(db_pool.get_ref())
        .get_by_day(campaign_id.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::Ok().json(stat))
}

#[utoipa::path(
    get,
    path = "/stats/advertisers/{advertiser_id}/campaigns",
    tag = "Stats",
    responses(
        (status = 200, description = "Got stat", body = domain::schemas::StatResponse),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::get("/advertisers/{advertiser_id}/campaigns")]
#[tracing::instrument(name = "Get stat campaign by id", skip(db_pool))]
pub async fn stat_advertisers_handler(
    advertiser_id: actix_web::web::Path<uuid::Uuid>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let stat = domain::usecase::StatCampaignUsecase::new(db_pool.get_ref())
        .get_with_advertisers(advertiser_id.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::Ok().json(stat))
}

#[utoipa::path(
    get,
    path = "/stats/advertisers/{advertiser_id}/campaigns/daily",
    tag = "Stats",
    responses(
        (status = 200, description = "Got stat", body = Vec<domain::schemas::StatDailyResponse>),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::get("/advertisers/{advertiser_id}/campaigns/daily")]
#[tracing::instrument(name = "Get stat campaign by id", skip(db_pool))]
pub async fn stat_advertisers_daily_handler(
    advertiser_id: actix_web::web::Path<uuid::Uuid>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let stat: Vec<domain::schemas::StatDailyResponse> = domain::usecase::StatCampaignUsecase::new(db_pool.get_ref())
        .get_with_advertisers_by_day(advertiser_id.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::Ok().json(stat))
}
