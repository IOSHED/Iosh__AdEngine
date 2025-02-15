use crate::{domain, infrastructure, interface};
pub mod campaigns;

pub fn advertisers_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .service(advertiser_bulk_handler)
        .service(advertiser_by_id_handler)
        .service(campaigns::campaigns_scope("/{advertiser_id}/campaigns"))
}

#[utoipa::path(
    post,
    path = "/advertisers/bulk",
    tag = "Advertiser",
    request_body = Vec<domain::schemas::AdvertiserProfileSchema>,
    responses(
        (status = 201, description = "Bulk advertiser creation", body = Vec<domain::schemas::AdvertiserProfileSchema>),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::post("/bulk")]
#[tracing::instrument(name = "create bulk advertisers", skip(db_pool))]
pub async fn advertiser_bulk_handler(
    register_data: actix_web::web::Json<Vec<domain::schemas::AdvertiserProfileSchema>>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let regsiters_user = domain::usecase::AdvertiserBulkRegisterUsecase::new(db_pool.get_ref())
        .registers(register_data.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::Created().json(regsiters_user))
}

#[utoipa::path(
    get,
    path = "/advertisers/{advertiser_id}",
    tag = "Advertiser", 
    responses(
        (status = 200, description = "Get advertiser by id", body = domain::schemas::AdvertiserProfileSchema),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 401, description = "Not found", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse),
    )
)]
#[actix_web::get("/{advertiser_id}")]
#[tracing::instrument(name = "Get advertiser by id", skip(db_pool))]
pub async fn advertiser_by_id_handler(
    advertiser_id: actix_web::web::Path<uuid::Uuid>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let user = domain::usecase::AdvertiserProfileUsecase::new(db_pool.get_ref())
        .get_by_id(advertiser_id.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::Ok().json(user))
}
