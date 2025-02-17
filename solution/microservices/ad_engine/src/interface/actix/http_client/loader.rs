use futures_util::TryStreamExt;

pub async fn loader_files(
    payload: actix_web::web::Payload,
    req: actix_web::HttpRequest,
    max_file_size: usize,
    allowed_mime_types: Vec<String>,
) -> Result<Vec<(String, Vec<u8>, String)>, actix_multipart::MultipartError> {
    let mut multipart = actix_multipart::Multipart::new(req.headers(), payload);
    let mut files = Vec::new();

    while let Some(mut field) = multipart.try_next().await? {
        let content_type = field
            .content_type()
            .ok_or(actix_multipart::MultipartError::ContentTypeMissing)?
            .to_string();

        let file_name = field
            .content_disposition()
            .ok_or(actix_multipart::MultipartError::ContentDispositionMissing)?
            .get_filename()
            .ok_or(actix_multipart::MultipartError::ContentDispositionNameMissing)?
            .to_string();

        if !allowed_mime_types.contains(&content_type) {
            return Err(actix_multipart::MultipartError::NotConsumed);
        }

        let mut bytes = actix_web::web::BytesMut::new();
        while let Some(chunk) = field.try_next().await? {
            if (bytes.len() + chunk.len()) > max_file_size {
                return Err(actix_multipart::MultipartError::NotConsumed);
            }
            bytes.extend_from_slice(&chunk);
        }

        files.push((file_name, bytes.to_vec(), content_type));
    }

    Ok(files)
}
