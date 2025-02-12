#[derive(utoipa::OpenApi)]
#[openapi(
    servers(
        (url = "/api", description = "API Server")
    ),
    paths(
        super::super::routers::healthcheck::healthcheck_handler,
        super::super::routers::time::time_advance_handler,
        super::super::routers::ml_score::ml_score_handler,
        super::super::routers::client::client_bulk_handler,
        super::super::routers::client::client_by_id_handler,
        super::super::routers::advertisers::advertiser_bulk_handler,
        super::super::routers::advertisers::advertiser_by_id_handler,
    ),
)]
pub struct ApiDocSwagger;
