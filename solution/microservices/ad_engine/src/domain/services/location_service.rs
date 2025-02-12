//! Location Service Module
//!
//! This module provides functionality for reverse geocoding coordinates into
//! city and country information using the OpenStreetMap Nominatim API.

use crate::domain;

/// A service for retrieving location information from coordinates
///
/// Uses the Nominatim OpenStreetMap API to perform reverse geocoding of
/// latitude/longitude coordinates into city and country information.
#[derive(std::fmt::Debug)]
pub struct LocationService {
    /// Base URL for the Nominatim API reverse geocoding endpoint
    base_url: &'static str,
}

impl LocationService {
    /// Creates a new instance of LocationService
    ///
    /// # Returns
    ///
    /// A new LocationService instance configured with the Nominatim API
    /// endpoint
    pub fn new() -> Self {
        LocationService {
            base_url: "https://nominatim.openstreetmap.org/reverse",
        }
    }

    /// Retrieves the city and country code for given coordinates
    ///
    /// # Arguments
    ///
    /// * `latitude` - The latitude coordinate
    /// * `longitude` - The longitude coordinate
    ///
    /// # Returns
    ///
    /// A tuple containing (city, country_code) as strings. Returns
    /// ("Неизвестно", "Неизвестно") if lookup fails
    #[tracing::instrument(name = "`LocationService` getting city and country by coords")]
    pub async fn get_city_and_country_code(&self, latitude: f64, longitude: f64) -> (String, String) {
        match self.fetch_location_info(latitude, longitude).await {
            Ok((city, country)) => (city, country),
            Err(_) => ("Неизвестно".to_string(), "Неизвестно".to_string()),
        }
    }

    /// Fetches location information from the Nominatim API
    ///
    /// # Arguments
    ///
    /// * `latitude` - The latitude coordinate
    /// * `longitude` - The longitude coordinate
    ///
    /// # Returns
    ///
    /// Result containing tuple of (city, country_code) or ServiceError
    async fn fetch_location_info(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> domain::services::ServiceResult<(String, String)> {
        let client = reqwest::Client::new();
        let params = self.get_params_query(latitude, longitude);

        let response = client
            .get(self.base_url)
            .query(&params)
            .header("User-Agent", "TravelService/1.0")
            .send()
            .await
            .map_err(|_| domain::services::ServiceError::Unknown)?;

        let data: serde_json::Value = response
            .json()
            .await
            .map_err(|_| domain::services::ServiceError::Unknown)?;
        self.parse_response(data)
    }

    /// Builds query parameters for the Nominatim API request
    ///
    /// # Arguments
    ///
    /// * `latitude` - The latitude coordinate
    /// * `longitude` - The longitude coordinate
    ///
    /// # Returns
    ///
    /// HashMap containing the required query parameters
    fn get_params_query(&self, latitude: f64, longitude: f64) -> std::collections::HashMap<&'static str, String> {
        let mut params = std::collections::HashMap::new();
        params.insert("lat", latitude.to_string());
        params.insert("lon", longitude.to_string());
        params.insert("format", "jsonv2".to_string());
        params
    }

    /// Parses the JSON response from Nominatim API
    ///
    /// # Arguments
    ///
    /// * `data` - JSON response data from the API
    ///
    /// # Returns
    ///
    /// Result containing tuple of (city, country_code) or ServiceError
    ///
    /// # Notes
    ///
    /// City name is extracted by checking multiple possible keys in order:
    /// city, town, village, municipality
    fn parse_response(&self, data: serde_json::Value) -> domain::services::ServiceResult<(String, String)> {
        let address = data
            .get("address")
            .ok_or("Address not found")
            .map_err(|_| domain::services::ServiceError::Unknown)?;

        let city_keys = ["city", "town", "village", "municipality"];
        let city = city_keys
            .iter()
            .filter_map(|&key| address.get(key))
            .next()
            .unwrap_or(&serde_json::Value::String("Неизвестно".to_string()))
            .as_str()
            .unwrap_or("Неизвестно")
            .to_string();

        let country = address
            .get("country_code")
            .unwrap_or(&serde_json::Value::String("Неизвестно".to_string()))
            .as_str()
            .unwrap_or("Неизвестно")
            .to_string();

        Ok((city, country))
    }
}
