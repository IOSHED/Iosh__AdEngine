use prometheus::Encoder;

use crate::infrastructure;

#[derive(Debug)]
pub struct PrometheusService;

impl PrometheusService {
    pub async fn get_metrics() -> Vec<u8> {
        let encoder = prometheus::TextEncoder::new();
        let mut buffer = vec![];
        let metric_families = prometheus::gather();
        encoder.encode(&metric_families, &mut buffer).unwrap_or_else(|_| {
            buffer.append(&mut b"Error in encoder metrics".into_iter().map(|b| *b).collect::<Vec<u8>>())
        });

        buffer
    }

    pub fn increment_campaign_created(time_advance: u32) {
        let metrics = infrastructure::metrics::prometheus::APP_METRICS.lock().unwrap();
        metrics
            .campaigns_created
            .with_label_values(&[&time_advance.to_string()])
            .inc();
    }

    pub fn increment_campaign_deleted(time_advance: u32) {
        let metrics = infrastructure::metrics::prometheus::APP_METRICS.lock().unwrap();
        metrics
            .campaigns_deleted
            .with_label_values(&[&time_advance.to_string()])
            .inc();
    }

    pub fn increment_campaign_updated(time_advance: u32) {
        let metrics = infrastructure::metrics::prometheus::APP_METRICS.lock().unwrap();
        metrics
            .campaigns_updated
            .with_label_values(&[&time_advance.to_string()])
            .inc();
    }

    pub fn increment_ads_visits(time_advance: u32) {
        let metrics = infrastructure::metrics::prometheus::APP_METRICS.lock().unwrap();
        metrics.ads_visits.with_label_values(&[&time_advance.to_string()]).inc();
    }

    pub fn increment_ads_clicks(time_advance: u32) {
        let metrics = infrastructure::metrics::prometheus::APP_METRICS.lock().unwrap();
        metrics.ads_clicks.with_label_values(&[&time_advance.to_string()]).inc();
    }

    pub fn add_total_clients(value: i64) {
        let metrics = infrastructure::metrics::prometheus::APP_METRICS.lock().unwrap();
        metrics.total_clients.add(value);
    }

    pub fn add_total_advertisers(value: i64) {
        let metrics = infrastructure::metrics::prometheus::APP_METRICS.lock().unwrap();
        metrics.total_advertisers.add(value);
    }

    pub fn increment_http_requests_total(request_method: &str, request_path: &str) {
        let metrics = infrastructure::metrics::prometheus::APP_METRICS.lock().unwrap();
        metrics
            .http_requests_total
            .with_label_values(&[request_method, request_path])
            .inc();
    }

    pub fn observe_http_response_time(value: f64, request_method: &str, request_path: &str) {
        let metrics = infrastructure::metrics::prometheus::APP_METRICS.lock().unwrap();
        metrics
            .http_response_time
            .with_label_values(&[request_method, request_path])
            .observe(value);
    }
}
