use crate::{domain, infrastructure, interface};

pub fn images_scope(path: &str) -> actix_web::Scope {
    actix_web::web::scope(path)
        .service(upload_image_campaign_handler)
        .service(get_campaign_image_handler)
        .service(get_campaign_name_images_handler)
        .service(delete_campaign_image_handler)
}

#[utoipa::path(
    post,
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}/images",
    tag = "Images",
    params(
        ("advertiser_id" = uuid::Uuid, Path, description = "Unique identifier for advertiser"),
        ("campaign_id" = uuid::Uuid, Path, description = "Unique identifier for campaign")
    ),
    request_body(
        description = "Upload images for campaign",
        content_type = "multipart/form-data",
        content = Vec<u8>
    ),
    responses(
        (status = 204, description = "Images successfully uploaded", body = ()),
        (status = 400, description = "Bad request - Invalid file format or size", body = interface::actix::exception::ExceptionResponse),
        (status = 404, description = "Advertiser or campaign not found", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::post("")]
pub async fn upload_image_campaign_handler(
    path_param: actix_web::web::Path<(uuid::Uuid, uuid::Uuid)>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
    app_state: actix_web::web::Data<domain::configurate::AppState>,
    request: actix_web::HttpRequest,
    payload: actix_web::web::Payload,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let (advertiser_id, campaign_id) = path_param.into_inner();

    let files: Vec<(String, Vec<u8>, String)> = interface::actix::http_client::loader_files(
        payload,
        request,
        app_state.media_max_size,
        app_state.media_support_mime.clone(),
    )
    .await
    .map_err(|e| domain::services::ServiceError::PayloadError(e.to_string()))?;

    domain::usecase::CampaignsUploadImageUsecase::new(db_pool.get_ref(), app_state.media_max_image_on_campaign)
        .upload(advertiser_id, campaign_id, files)
        .await?;

    Ok(actix_web::HttpResponse::NoContent().into())
}

#[utoipa::path(
    get,
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}/images/{file_name}",
    tag = "Images",
    params(
        ("advertiser_id" = uuid::Uuid, Path, description = "Unique identifier for advertiser"),
        ("campaign_id" = uuid::Uuid, Path, description = "Unique identifier for campaign"),
        ("file_name" = String, Path, description = "Name of the image file to retrieve")
    ),
    responses(
        (status = 200, description = "Image found and returned successfully", content_type = "image/*", body = Vec<u8>, example = "[255, 216, 255]"),
        (status = 404, description = "Image, advertiser or campaign not found", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::get("/{file_name}")]
pub async fn get_campaign_image_handler(
    path_param: actix_web::web::Path<(uuid::Uuid, uuid::Uuid, String)>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let (advertiser_id, campaign_id, file_name) = path_param.into_inner();

    let (mime_type, data) = domain::usecase::CampaignsGetImageUsecase::new(db_pool.get_ref())
        .get(advertiser_id, campaign_id, file_name)
        .await?;

    Ok(actix_web::HttpResponse::Ok().content_type(mime_type).body(data))
}


#[utoipa::path(
    delete,
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}/images/{file_name}",
    tag = "Images",
    params(
        ("advertiser_id" = uuid::Uuid, Path, description = "Unique identifier for advertiser"),
        ("campaign_id" = uuid::Uuid, Path, description = "Unique identifier for campaign"),
        ("file_name" = String, Path, description = "Name of the image file for deleting")
    ),
    responses(
        (status = 204, description = "Image found and returned successfully", body = ()),
        (status = 404, description = "Image, advertiser or campaign not found", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::delete("/{file_name}")]
pub async fn delete_campaign_image_handler(
    path_param: actix_web::web::Path<(uuid::Uuid, uuid::Uuid, String)>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let (advertiser_id, campaign_id, file_name) = path_param.into_inner();

   domain::usecase::CampaignsDeleteImageUsecase::new(db_pool.get_ref())
        .delete(advertiser_id, campaign_id, file_name)
        .await?;

    Ok(actix_web::HttpResponse::NoContent().into())
}


#[utoipa::path(
    get,
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}/images",
    tag = "Images",
    params(
        ("advertiser_id" = uuid::Uuid, Path, description = "Unique identifier for advertiser"),
        ("campaign_id" = uuid::Uuid, Path, description = "Unique identifier for campaign")
    ),
    responses(
        (status = 200, description = "Successfully retrieved list of image names for the campaign", body = Vec<String>),
        (status = 404, description = "Advertiser or campaign not found", body = interface::actix::exception::ExceptionResponse),
        (status = 500, description = "Internal server error", body = interface::actix::exception::ExceptionResponse)
    )
)]
#[actix_web::get("")]
pub async fn get_campaign_name_images_handler(
    path_param: actix_web::web::Path<(uuid::Uuid, uuid::Uuid)>,
    db_pool: actix_web::web::Data<infrastructure::database_connection::sqlx_lib::SqlxPool>,
) -> interface::actix::ActixResult<actix_web::HttpResponse> {
    let (advertiser_id, campaign_id) = path_param.into_inner();

    let images = domain::usecase::CampaignsGetNameImagesUsecase::new(db_pool.get_ref())
        .get(advertiser_id, campaign_id)
        .await?;

    Ok(actix_web::HttpResponse::Ok().json(images))
}
