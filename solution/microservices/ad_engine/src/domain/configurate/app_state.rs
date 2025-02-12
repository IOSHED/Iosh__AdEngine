use crate::infrastructure;

/// Represents the application's state that can be shared across different parts
/// of the system.
///
/// This struct is designed to hold any shared resources or configuration that
/// needs to be accessible throughout the application's lifetime.
#[derive(Clone)]
pub struct AppState {}

impl From<&infrastructure::configurate::Config> for AppState {
    /// Creates a new `AppState` instance from a configuration reference.
    ///
    /// # Arguments
    ///
    /// * `_config` - A reference to the infrastructure configuration
    ///
    /// # Returns
    ///
    /// Returns a new instance of `AppState`
    fn from(_config: &infrastructure::configurate::Config) -> Self {
        Self {}
    }
}
