use crate::domain;

#[derive(Debug)]
pub struct AggregateStatService;

impl AggregateStatService {
    pub fn calculate_total_stats(&self, stats: &[domain::schemas::StatDailyResponse]) -> (u32, u32, f64, f64) {
        stats.iter().fold((0, 0, 0.0, 0.0), |(imp, clk, si, sc), s| {
            (
                imp + s.impressions_count,
                clk + s.clicks_count,
                si + s.spent_impressions,
                sc + s.spent_clicks,
            )
        })
    }

    pub fn calculate_conversion(&self, impressions: u32, clicks: u32) -> f64 {
        if impressions > 0 {
            (clicks as f64 / impressions as f64) * 100.0
        } else {
            0.0
        }
    }

    pub fn aggregate_daily_stats(
        &self,
        stats: Vec<Vec<domain::schemas::StatDailyResponse>>,
    ) -> Vec<domain::schemas::StatDailyResponse> {
        let mut aggregated = std::collections::HashMap::new();

        for daily in stats.into_iter().flatten() {
            let entry = aggregated.entry(daily.date).or_insert_with(|| daily.clone());
            entry.clicks_count += daily.clicks_count;
            entry.impressions_count += daily.impressions_count;
            entry.spent_clicks += daily.spent_clicks;
            entry.spent_impressions += daily.spent_impressions;
            entry.spent_total += daily.spent_total;
            entry.conversion = self.calculate_conversion(entry.impressions_count, entry.clicks_count);
        }

        let mut result: Vec<_> = aggregated.into_values().collect();

        result.sort_by(|a, b| b.date.cmp(&a.date));
        result
    }

    pub fn create_stat_response(
        &self,
        impressions: u32,
        clicks: u32,
        spent_imp: f64,
        spent_clk: f64,
    ) -> domain::schemas::StatResponse {
        domain::schemas::StatResponse {
            impressions_count: impressions,
            clicks_count: clicks,
            spent_impressions: spent_imp,
            spent_clicks: spent_clk,
            spent_total: spent_clk + spent_imp,
            conversion: self.calculate_conversion(impressions, clicks),
        }
    }
}
