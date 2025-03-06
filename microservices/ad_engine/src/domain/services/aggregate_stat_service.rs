use crate::domain;

/// Service for aggregating and calculating statistics from daily responses
///
/// This service provides methods to process and aggregate statistical data
/// related to impressions, clicks, and conversions.
#[derive(Debug)]
pub struct AggregateStatService;

impl AggregateStatService {
    /// Calculates total statistics from a slice of daily stat responses
    ///
    /// # Arguments
    /// * `stats` - Slice of StatDailyResponse objects to process
    ///
    /// # Returns
    /// Tuple containing:
    /// * Total impression count (u32)
    /// * Total click count (u32)
    /// * Total spent on impressions (f64)
    /// * Total spent on clicks (f64)
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

    /// Calculates conversion rate as a percentage
    ///
    /// # Arguments
    /// * `impressions` - Number of impressions
    /// * `clicks` - Number of clicks
    ///
    /// # Returns
    /// Conversion rate as percentage (clicks/impressions * 100)
    /// Returns 0.0 if impressions is 0
    pub fn calculate_conversion(&self, impressions: u32, clicks: u32) -> f64 {
        if impressions > 0 {
            (clicks as f64 / impressions as f64) * 100.0
        } else {
            0.0
        }
    }

    /// Aggregates multiple vectors of daily statistics into a single sorted
    /// vector
    ///
    /// # Arguments
    /// * `stats` - Vector of vectors containing StatDailyResponse objects
    ///
    /// # Returns
    /// Vector of aggregated StatDailyResponse objects sorted by date descending
    ///
    /// # Details
    /// - Combines statistics for matching dates
    /// - Recalculates conversion rates for aggregated entries
    /// - Returns results sorted with most recent dates first
    pub fn aggregate_daily_stats(
        &self,
        stats: Vec<Vec<domain::schemas::StatDailyResponse>>,
    ) -> Vec<domain::schemas::StatDailyResponse> {
        let mut aggregated = std::collections::HashMap::new();

        for daily in stats.into_iter().flatten() {
            let entry = aggregated.entry(daily.date.clone()).or_insert(daily.clone());
            if entry.impressions_count != daily.impressions_count || entry.clicks_count != daily.clicks_count {
                entry.clicks_count += daily.clicks_count;
                entry.impressions_count += daily.impressions_count;
                entry.spent_clicks += daily.spent_clicks;
                entry.spent_impressions += daily.spent_impressions;
                entry.spent_total += daily.spent_total;
                entry.conversion = self.calculate_conversion(entry.impressions_count, entry.clicks_count);
            }
        }

        let mut result: Vec<_> = aggregated.into_values().collect();
        result.sort_by(|a, b| b.date.cmp(&a.date));
        result
    }

    /// Creates a new StatResponse from individual statistics
    ///
    /// # Arguments
    /// * `impressions` - Number of impressions
    /// * `clicks` - Number of clicks
    /// * `spent_imp` - Amount spent on impressions
    /// * `spent_clk` - Amount spent on clicks
    ///
    /// # Returns
    /// New StatResponse object with calculated totals and conversion rate
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

#[cfg(test)]
mod tests {
    use domain::schemas::StatDailyResponse;

    use super::*;

    #[test]
    fn test_calculate_total_stats() {
        let service = AggregateStatService;

        let stats = vec![
            StatDailyResponse {
                impressions_count: 100,
                clicks_count: 10,
                spent_impressions: 5.0,
                spent_clicks: 1.0,
                date: 1,
                conversion: 10.0,
                spent_total: 6.0,
            },
            StatDailyResponse {
                impressions_count: 200,
                clicks_count: 20,
                spent_impressions: 8.0,
                spent_clicks: 2.0,
                date: 2,
                conversion: 10.0,
                spent_total: 10.0,
            },
        ];

        let (total_imp, total_clk, total_spent_imp, total_spent_clk) = service.calculate_total_stats(&stats);

        assert_eq!(total_imp, 300);
        assert_eq!(total_clk, 30);
        assert_eq!(total_spent_imp, 13.0);
        assert_eq!(total_spent_clk, 3.0);
    }

    #[test]
    fn test_calculate_conversion() {
        let service = AggregateStatService;

        let conversion_with_impressions = service.calculate_conversion(100, 10);
        assert_eq!(conversion_with_impressions, 10.0);

        let conversion_without_impressions = service.calculate_conversion(0, 10);
        assert_eq!(conversion_without_impressions, 0.0);
    }

    #[test]
    fn test_aggregate_daily_stats() {
        let service = AggregateStatService;

        let stats_vec = vec![
            vec![StatDailyResponse {
                impressions_count: 100,
                clicks_count: 10,
                spent_impressions: 5.0,
                spent_clicks: 1.0,
                date: 1,
                conversion: 10.0,
                spent_total: 6.0,
            }],
            vec![
                StatDailyResponse {
                    impressions_count: 300,
                    clicks_count: 20,
                    spent_impressions: 8.0,
                    spent_clicks: 2.0,
                    date: 2,
                    conversion: 10.0,
                    spent_total: 10.0,
                },
                StatDailyResponse {
                    impressions_count: 100,
                    clicks_count: 15,
                    spent_impressions: 3.0,
                    spent_clicks: 1.5,
                    date: 1,
                    conversion: 15.0,
                    spent_total: 4.5,
                },
            ],
        ];

        let aggregated_stats = service.aggregate_daily_stats(stats_vec);

        assert_eq!(aggregated_stats.len(), 2);
        assert_eq!(aggregated_stats[0].date, 2);
        assert_eq!(aggregated_stats[0].impressions_count, 300);
        assert_eq!(aggregated_stats[0].clicks_count, 20);

        assert_eq!(aggregated_stats[1].date, 1);
        assert_eq!(aggregated_stats[1].impressions_count, 200);
        assert_eq!(aggregated_stats[1].clicks_count, 25);
    }

    #[test]
    fn test_create_stat_response() {
        let service = AggregateStatService;

        let response = service.create_stat_response(100, 10, 5.0, 1.0);

        assert_eq!(response.impressions_count, 100);
        assert_eq!(response.clicks_count, 10);
        assert_eq!(response.spent_impressions, 5.0);
        assert_eq!(response.spent_clicks, 1.0);
        assert_eq!(response.spent_total, 6.0);
        assert_eq!(response.conversion, 10.0);
    }
}
