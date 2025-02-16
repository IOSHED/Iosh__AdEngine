use lazy_static::lazy_static;

lazy_static! {
    pub static ref APP_METRICS: std::sync::Mutex<AppMetrics> = std::sync::Mutex::new(AppMetrics::new());
}

pub struct AppMetrics {
    pub campaigns_created: prometheus::IntCounterVec,
    pub campaigns_deleted: prometheus::IntCounterVec,
    pub campaigns_updated: prometheus::IntCounterVec,
    pub ads_visits: prometheus::IntCounterVec,
    pub ads_clicks: prometheus::IntCounterVec,
    pub total_clients: prometheus::IntGauge,
    pub total_advertisers: prometheus::IntGauge,

    pub http_requests_total: prometheus::IntCounterVec,
    pub http_response_time: prometheus::HistogramVec,
}

impl AppMetrics {
    fn new() -> Self {
        Self {
            campaigns_created: prometheus::register_int_counter_vec!(
                prometheus::opts!("campaigns_created_total", "Total created campaigns",),
                &["time_advance"],
            )
            .expect("Failed create metric campaigns_created".into()),

            campaigns_deleted: prometheus::register_int_counter_vec!(
                prometheus::opts!("campaigns_deleted_total", "Total deleted campaigns",),
                &["time_advance"],
            )
            .expect("Failed create metric campaigns_deleted".into()),

            campaigns_updated: prometheus::register_int_counter_vec!(
                prometheus::opts!("campaigns_updated_total", "Total updated campaigns",),
                &["time_advance"],
            )
            .expect("Failed create metric campaigns_updated".into()),

            ads_visits: prometheus::register_int_counter_vec!(
                prometheus::opts!("ads_visits_total", "Total ads visits",),
                &["time_advance"],
            )
            .expect("Failed create metric ads_visits".into()),

            ads_clicks: prometheus::register_int_counter_vec!(
                prometheus::opts!("ads_clicks_total", "Total ads clicks",),
                &["time_advance"],
            )
            .expect("Failed create metric ads_clicks".into()),

            total_clients: prometheus::register_int_gauge!(prometheus::opts!(
                "total_clients",
                "Total number of clients",
            ))
            .expect("Failed create metric total_clients".into()),

            total_advertisers: prometheus::register_int_gauge!(prometheus::opts!(
                "total_advertisers",
                "Total number of advertisers",
            ))
            .expect("Failed create metric total_advertisers".into()),

            http_requests_total: prometheus::register_int_counter_vec!(
                prometheus::opts!("http_requests_total", "HTTP requests total"),
                &["method", "path"]
            )
            .expect("Failed create metric http_requests_total".into()),

            http_response_time: prometheus::register_histogram_vec!(
                "http_response_time_seconds",
                "HTTP response times",
                &["method", "path"],
                vec![0.01, 0.05, 0.1, 0.3, 0.5, 1.0, 2.5, 5.0,]
            )
            .expect("Failed create metric http_response_time".into()),
        }
    }
}
