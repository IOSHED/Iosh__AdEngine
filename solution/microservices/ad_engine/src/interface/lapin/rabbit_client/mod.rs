//! RabbitMQ Server Implementation
//!
//! This module provides a robust RabbitMQ server implementation that handles
//! message routing and processing. It supports topic-based routing, durable
//! queues and exchanges, and asynchronous message handling.

use async_trait::async_trait;
use futures_lite::stream::StreamExt;

pub mod context;
pub mod route;

pub use context::RabbitContext;
pub use route::MessageHandler;

use crate::{domain, infrastructure, interface};

/// Core RabbitMQ server struct that manages connections, channels and message
/// routing
pub struct RabbitServer {
    /// RabbitMQ connection configuration
    config: infrastructure::configurate::RabbitMqConfig,
    /// Application state shared across handlers
    app_state: domain::configurate::AppState,
    /// Database connection pool
    connection_pool: sqlx::Pool<sqlx::Postgres>,
    /// Registered message routes and their handlers
    routes: std::collections::HashMap<String, route::Route>,
}

impl RabbitServer {
    /// Creates a new RabbitServer instance
    ///
    /// # Arguments
    ///
    /// * `config` - RabbitMQ configuration that can be converted into
    ///   RabbitMqConfig
    /// * `app_state` - Application state to be shared with handlers
    /// * `connection_pool` - PostgreSQL connection pool
    ///
    /// # Returns
    ///
    /// Returns a new RabbitServer instance with initialized routes
    pub fn new(
        config: impl Into<infrastructure::configurate::RabbitMqConfig>,
        app_state: domain::configurate::AppState,
        connection_pool: sqlx::Pool<sqlx::Postgres>,
    ) -> Self {
        Self {
            config: config.into(),
            app_state,
            connection_pool,
            routes: Self::register_core_routes(),
        }
    }

    /// Registers core application routes with their corresponding handlers
    ///
    /// # Returns
    ///
    /// Returns a HashMap containing route patterns mapped to their handlers
    fn register_core_routes() -> std::collections::HashMap<String, route::Route> {
        let mut routes = std::collections::HashMap::new();
        routes.insert(
            "user.create".into(),
            route::Route::new(interface::lapin::routers::UserCreate),
        );
        routes.insert(
            "user.are_exist".into(),
            route::Route::new(interface::lapin::routers::UserAreExist),
        );
        routes
    }

    /// Sets up RabbitMQ infrastructure components including exchanges, queues
    /// and bindings
    ///
    /// # Arguments
    ///
    /// * `channel` - RabbitMQ channel to perform operations on
    ///
    /// # Returns
    ///
    /// Returns Result indicating success or failure of setup
    async fn setup_infrastructure(
        &self,
        channel: &lapin::Channel,
    ) -> Result<(), interface::lapin::exception::ServerError> {
        self.declare_exchange(channel)
            .await
            .map_err(|e| interface::lapin::exception::ServerError::SetupError(e.to_string()))?;

        self.declare_queue(channel)
            .await
            .map_err(|e| interface::lapin::exception::ServerError::SetupError(e.to_string()))?;

        self.bind_routes(channel)
            .await
            .map_err(|e| interface::lapin::exception::ServerError::SetupError(e.to_string()))?;

        Ok(())
    }

    /// Declares the topic exchange used for message routing
    ///
    /// # Arguments
    ///
    /// * `channel` - RabbitMQ channel
    ///
    /// # Returns
    ///
    /// Returns Result indicating success or failure of exchange declaration
    async fn declare_exchange(&self, channel: &lapin::Channel) -> Result<(), lapin::Error> {
        channel
            .exchange_declare(
                &self.config.exchange,
                lapin::ExchangeKind::Topic,
                lapin::options::ExchangeDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                lapin::types::FieldTable::default(),
            )
            .await
    }

    /// Declares the queue that will receive messages
    ///
    /// # Arguments
    ///
    /// * `channel` - RabbitMQ channel
    ///
    /// # Returns
    ///
    /// Returns Result indicating success or failure of queue declaration
    async fn declare_queue(&self, channel: &lapin::Channel) -> Result<(), lapin::Error> {
        channel
            .queue_declare(
                &self.config.queue_name,
                lapin::options::QueueDeclareOptions {
                    durable: true,
                    ..Default::default()
                },
                lapin::types::FieldTable::default(),
            )
            .await?;
        Ok(())
    }

    /// Binds routes to the queue with their corresponding patterns
    ///
    /// # Arguments
    ///
    /// * `channel` - RabbitMQ channel
    ///
    /// # Returns
    ///
    /// Returns Result indicating success or failure of route binding
    async fn bind_routes(&self, channel: &lapin::Channel) -> Result<(), lapin::Error> {
        for (pattern, _route) in &self.routes {
            channel
                .queue_bind(
                    &self.config.queue_name,
                    &self.config.exchange,
                    pattern,
                    lapin::options::QueueBindOptions::default(),
                    lapin::types::FieldTable::default(),
                )
                .await?;
        }
        Ok(())
    }

    /// Creates a new RabbitMQ channel from the connection
    ///
    /// # Returns
    ///
    /// Returns Result containing the channel or connection error
    async fn create_channel(&self) -> Result<lapin::Channel, interface::lapin::exception::ServerError> {
        let uri = format!(
            "amqp://{}:{}@{}:{}/{}",
            self.config.username, self.config.password, self.config.host, self.config.port, self.config.vhost
        );

        lapin::Connection::connect(&uri, lapin::ConnectionProperties::default())
            .await
            .map_err(interface::lapin::exception::ServerError::ConnectionError)?
            .create_channel()
            .await
            .map_err(interface::lapin::exception::ServerError::ConnectionError)
    }

    /// Processes a single message with its corresponding handler
    ///
    /// # Arguments
    ///
    /// * `delivery` - The delivered message
    /// * `context` - Shared context for message handling
    /// * `handler` - Optional message handler for the route
    async fn process_message(
        delivery: lapin::message::Delivery,
        context: std::sync::Arc<context::RabbitContext>,
        handler: Option<Box<dyn route::MessageHandler + Send + Sync>>,
    ) {
        let routing_key = delivery.routing_key.clone();

        match handler {
            Some(handler) => match handler.handle(&context, &delivery.data, &delivery).await {
                Ok(_) =>
                    if let Err(e) = delivery.ack(lapin::options::BasicAckOptions::default()).await {
                        tracing::error!("Failed to ack message {}: {}", routing_key, e);
                    },
                Err(e) => {
                    tracing::error!("Handler error for {}: {}", routing_key, e);
                    if let Err(e) = delivery.nack(lapin::options::BasicNackOptions::default()).await {
                        tracing::error!("Failed to nack message {}: {}", routing_key, e);
                    }
                },
            },
            None => {
                tracing::warn!("No handler for routing key: {}", routing_key);
                if let Err(e) = delivery.nack(lapin::options::BasicNackOptions::default()).await {
                    tracing::error!("Failed to nack message {}: {}", routing_key, e);
                }
            },
        }
    }

    /// Runs the main message processing loop
    ///
    /// # Arguments
    ///
    /// * `consumer` - RabbitMQ message consumer
    /// * `context` - Shared context for message handling
    /// * `routes` - Registered message routes
    ///
    /// # Returns
    ///
    /// Returns Result indicating success or failure of message loop
    async fn run_message_loop(
        mut consumer: lapin::Consumer,
        context: std::sync::Arc<context::RabbitContext>,
        routes: std::collections::HashMap<String, route::Route>,
    ) -> Result<(), interface::lapin::exception::ServerError> {
        while let Some(delivery) = consumer.next().await {
            let delivery = match delivery {
                Ok(d) => d,
                Err(e) => {
                    tracing::error!("Consumer error: {}", e);
                    continue;
                },
            };

            let routing_key = delivery.routing_key.clone();
            let handler = routes.get(routing_key.as_str()).map(|r| r.handler.boxed_clone());

            let context_clone = context.clone();
            tokio::spawn(async move {
                Self::process_message(delivery, context_clone, handler).await;
            });
        }

        Ok(())
    }
}

#[async_trait]
impl interface::IServer for RabbitServer {
    type ErrorLaunch = Box<dyn std::error::Error>;

    /// Launches the RabbitMQ server and starts processing messages
    ///
    /// # Returns
    ///
    /// Returns Result indicating success or failure of server launch
    async fn launch(mut self) -> Result<(), Self::ErrorLaunch> {
        let channel = self.create_channel().await?;
        self.setup_infrastructure(&channel).await?;

        let consumer = channel
            .basic_consume(
                &self.config.queue_name,
                &self.config.consumer_tag,
                lapin::options::BasicConsumeOptions::default(),
                lapin::types::FieldTable::default(),
            )
            .await
            .map_err(interface::lapin::exception::ServerError::ConsumerError)?;

        let context = std::sync::Arc::new(context::RabbitContext::new(
            self.connection_pool,
            self.app_state,
            channel,
            self.config.clone(),
        ));

        tracing::info!("🚀 RabbitMQ server initialized on queue: {}", self.config.queue_name);

        Self::run_message_loop(consumer, context, self.routes)
            .await
            .map_err(|e| e.into())
    }
}
