#[derive(utoipa::OpenApi)]
#[openapi(
    servers(
        (url = "/api", description = "API Server")
    ),
    paths(
        super::super::routers::healthcheck::healthcheck_handler,
        super::super::routers::time::time_advance_handler,
    ),
)]
pub struct ApiDocSwagger;
