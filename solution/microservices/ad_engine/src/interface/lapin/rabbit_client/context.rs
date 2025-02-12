//! RabbitMQ context module for handling message publishing and state management
//!
//! This module provides the RabbitContext struct which encapsulates database
//! connection, application state, and RabbitMQ channel management.

use crate::{domain, infrastructure, interface};

/// Main context struct for RabbitMQ operations
///
/// Holds connection pools, application state and channel configuration needed
/// for interacting with RabbitMQ message broker.
pub struct RabbitContext {
    /// PostgreSQL connection pool
    pub db_pool: sqlx::Pool<sqlx::Postgres>,
    /// Application state containing runtime configuration
    pub app_state: domain::configurate::AppState,
    /// RabbitMQ channel for publishing messages
    channel: lapin::Channel,
    /// RabbitMQ specific configuration
    config: infrastructure::configurate::RabbitMqConfig,
}

impl RabbitContext {
    /// Creates a new RabbitContext instance
    ///
    /// # Arguments
    ///
    /// * `db_pool` - PostgreSQL connection pool
    /// * `app_state` - Application state containing runtime configuration
    /// * `channel` - Initialized RabbitMQ channel
    /// * `config` - RabbitMQ configuration parameters
    ///
    /// # Returns
    ///
    /// Returns a new RabbitContext instance initialized with the provided
    /// parameters
    pub fn new(
        db_pool: sqlx::Pool<sqlx::Postgres>,
        app_state: domain::configurate::AppState,
        channel: lapin::Channel,
        config: infrastructure::configurate::RabbitMqConfig,
    ) -> Self {
        Self {
            db_pool,
            app_state,
            channel,
            config,
        }
    }

    /// Publishes a message to RabbitMQ
    ///
    /// # Arguments
    ///
    /// * `routing_key` - The routing key for message delivery
    /// * `payload` - The message payload that implements Serialize
    ///
    /// # Returns
    ///
    /// Returns Ok(()) on successful publish, or ResponseError on failure
    ///
    /// # Errors
    ///
    /// Will return error if:
    /// - Message serialization fails
    /// - Publishing to RabbitMQ fails
    pub async fn send_response<T: serde::Serialize + ?Sized>(
        &self,
        routing_key: &str,
        payload: &T,
    ) -> Result<(), interface::lapin::ResponseError> {
        let serialized = serde_json::to_vec(payload).map_err(interface::lapin::ResponseError::Serialization)?;

        self.channel
            .basic_publish(
                &self.config.exchange,
                routing_key,
                lapin::options::BasicPublishOptions::default(),
                &serialized,
                lapin::BasicProperties::default(),
            )
            .await
            .map_err(interface::lapin::ResponseError::Publish)?;

        Ok(())
    }
}
