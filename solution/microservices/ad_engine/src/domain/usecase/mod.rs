//! # Usecase
//! Use case performs a specific business operation and can interact with
//! multiple services. It manages the logic that defines how data and operations
//! are related to each other.

mod time_advance_usecase;
mod user_register;

pub use time_advance_usecase::TimeAdvanceUsecase;
