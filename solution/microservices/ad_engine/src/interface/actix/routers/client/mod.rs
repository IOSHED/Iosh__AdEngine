use crate::{domain, infrastructure, interface};

pub fn client_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .service(client_bulk_handler)
        .service(client_by_id_handler)
}

#[utoipa::path(
    post,
    path = "/client/bulk",
    tag = "Client",
    request_body = Vec<domain::schemas::ClientProfileSchema>,
    responses(
        (status = 201, description = "Bulk client creation", body = Vec<domain::schemas::ClientProfileSchema>),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::post("/bulk")]
#[tracing::instrument(name = "client_bulk_handler", skip(db_pool, redis_pool, app_state))]
pub async fn client_bulk_handler(
    register_data: actix_web::web::Json<Vec<domain::schemas::ClientProfileSchema>>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
    redis_pool: actix_web::web::Data<infrastructure::database_connection::redis::RedisPool>,
    app_state: actix_web::web::Data<domain::configurate::AppState>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let regsiters_user =
        domain::usecase::ClientBulkRegisterUsecase::new(db_pool.get_ref(), redis_pool.get_ref(), app_state.get_ref())
            .registers(register_data.into_inner())
            .await?;

    Ok(actix_web::HttpResponse::Created().json(regsiters_user))
}

#[utoipa::path(
    get,
    path = "/client/{client_id}",
    tag = "Client", responses(
        (status = 200, description = "Get client by id", body = domain::schemas::ClientProfileSchema),
        (status = 400, description = "Bad request", body = interface::actix::exception::ExceptionResponse),
        (status = 401, description = "Not found", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse),
    )
)]
#[actix_web::get("/{client_id}")]
#[tracing::instrument(name = "client_by_id_handler", skip(db_pool))]
pub async fn client_by_id_handler(
    client_id: actix_web::web::Path<uuid::Uuid>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let user = domain::usecase::ClientProfileUsecase::new(db_pool.get_ref())
        .get_by_id(client_id.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::Ok().json(user))
}
