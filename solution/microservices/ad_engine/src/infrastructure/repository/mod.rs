//! # Repository
//! Repositories are responsible for interacting with data sources (for example,
//! databases, APIs, etc.). They implement a pattern of storing and providing
//! data.

mod error;
pub mod sqlx_lib;

pub use error::RepoError;

use crate::infrastructure;

pub type RepoResult<T> = Result<T, RepoError>;

pub trait IRepo<'p> {
    fn new(db_pool: &'p infrastructure::database_connection::sqlx_lib::SqlxPool) -> Self;
}
