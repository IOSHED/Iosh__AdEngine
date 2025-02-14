#[derive(utoipa::OpenApi)]
#[openapi(
    servers(
        (url = "/api", description = "API Server")
    ),
    paths(
        super::super::routers::healthcheck::healthcheck_handler,
        super::super::routers::time::time_advance_handler,
        super::super::routers::ml_score::ml_score_handler,
        super::super::routers::ads::ads_handler,
        super::super::routers::client::client_bulk_handler,
        super::super::routers::client::client_by_id_handler,
        super::super::routers::advertisers::advertiser_bulk_handler,
        super::super::routers::advertisers::advertiser_by_id_handler,
        super::super::routers::advertisers::campaigns::campaigns_create_handler,
        super::super::routers::advertisers::campaigns::campaigns_update_handler,
        super::super::routers::advertisers::campaigns::campaigns_delete_handler,
        super::super::routers::advertisers::campaigns::campaigns_get_by_id_handler,
        super::super::routers::advertisers::campaigns::campaigns_get_list_handler
    ),
)]
pub struct ApiDocSwagger;
