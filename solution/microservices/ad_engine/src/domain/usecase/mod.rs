//! # Usecase
//! Use case performs a specific business operation and can interact with
//! multiple services. It manages the logic that defines how data and operations
//! are related to each other.

mod user_are_exist;
mod user_register;

pub use user_are_exist::UserAreExistUsecase;
pub use user_register::UserRegisterUsecase;
