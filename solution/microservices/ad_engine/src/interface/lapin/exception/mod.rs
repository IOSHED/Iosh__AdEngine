/// Represents errors that can occur during response handling in the messaging
/// system.
///
/// This enum encapsulates two main categories of errors:
/// - Serialization errors when converting messages to/from JSON
/// - Publishing errors when sending messages to the message broker

#[derive(std::fmt::Debug)]
pub enum ResponseError {
    /// Error that occurs during JSON serialization/deserialization using
    /// serde_json
    Serialization(serde_json::Error),
    /// Error that occurs when publishing messages using the lapin AMQP client
    Publish(lapin::Error),
}

impl std::fmt::Display for ResponseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Serialization(e) => write!(f, "Serialization error: {}", e),
            Self::Publish(e) => write!(f, "Message publish error: {}", e),
        }
    }
}

/// Implements the standard error trait to allow using this in error handling
/// contexts
impl std::error::Error for ResponseError {}

/// Represents errors that can occur during server operations in the messaging
/// system.
///
/// This enum encapsulates different categories of errors that may occur during:
/// - Connection establishment with the message broker
/// - Infrastructure and resource setup
/// - Message consumption from queues
#[derive(Debug)]
pub enum ServerError {
    /// Error that occurs when establishing connection with the message broker
    ConnectionError(lapin::Error),
    /// Error during setup of messaging infrastructure (queues, exchanges etc)
    SetupError(String),
    /// Error that occurs while consuming messages from queues
    ConsumerError(lapin::Error),
}

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ConnectionError(e) => write!(f, "Connection failed: {}", e),
            Self::SetupError(e) => write!(f, "Infrastructure setup failed: {}", e),
            Self::ConsumerError(e) => write!(f, "Consumer error: {}", e),
        }
    }
}

impl std::error::Error for ServerError {}
