#[derive(utoipa::OpenApi)]
#[openapi(
    servers(
        (url = "/api", description = "API Server")
    ),
    paths(
        super::super::routers::healthcheck::healthcheck_handler,
    ),
)]
pub struct ApiDocSwagger;
