use crate::{domain, infrastructure, interface};

pub fn moderate_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .service(moderate_config_handler)
        .service(moderate_add_list_handler)
        .service(moderate_delete_list_handler)
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

#[utoipa::path(
    post,
    path = "/moderate/list",
    tag = "Moderate",
    request_body = Vec<String>,
    responses(
        (status = 204, description = "Set activate moderate to value", body = ()),
        (status = 400, description = "Bad activate moderate", body = interface::actix::exception::ExceptionResponse),
        (status = 409, description = "NOt unique", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::post("/list")]
pub async fn moderate_add_list_handler(
    new_word_to_list: actix_web::web::Json<Vec<String>>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
    redis_pool: actix_web::web::Data<infrastructure::database_connection::redis::RedisPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    domain::usecase::ModerateAddListUsecase::new(db_pool.get_ref(), redis_pool.get_ref())
        .add_list(new_word_to_list.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::NoContent().into())
}

#[utoipa::path(
    delete,
    path = "/moderate/list",
    tag = "Moderate",
    request_body = Vec<String>,
    responses(
        (status = 204, description = "Set activate moderate to value", body = ()),
        (status = 400, description = "Bad activate moderate", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::delete("/list")]
pub async fn moderate_delete_list_handler(
    new_delete_word_to_list: actix_web::web::Json<Vec<String>>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
    redis_pool: actix_web::web::Data<infrastructure::database_connection::redis::RedisPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    domain::usecase::ModerateDeleteListUsecase::new(db_pool.get_ref(), redis_pool.get_ref())
        .delete_list(new_delete_word_to_list.into_inner())
        .await?;

    Ok(actix_web::HttpResponse::NoContent().into())
}
