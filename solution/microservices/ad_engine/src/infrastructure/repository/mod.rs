//! # Repository
//! Repositories are responsible for interacting with data sources (for example,
//! databases, APIs, etc.). They implement a pattern of storing and providing
//! data.

mod error;
pub mod sqlx_lib;

pub use error::RepoError;

pub type RepoResult<T> = Result<T, RepoError>;
