//! # Usecase
//! Use case performs a specific business operation and can interact with
//! multiple services. It manages the logic that defines how data and operations
//! are related to each other.

mod client_bulk_register;
mod client_profile;
mod time_advance;

pub use client_bulk_register::ClientBulkRegisterUsecase;
pub use client_profile::ClientProfileUsecase;
pub use time_advance::TimeAdvanceUsecase;
