mod advertiser_repository;
mod client_repository;
mod ml_score_repository;
pub use advertiser_repository::{AdvertiserReturningSchema, PgAdvertiserRepository};
pub use client_repository::{ClientReturningSchema, PgClientRepository};
pub use ml_score_repository::PgScoreRepository;

impl From<sqlx::Error> for super::RepoError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(db_err) => {
                if db_err.is_unique_violation() {
                    return Self::UniqueConstraint(db_err.constraint().get_or_insert("object").to_string());
                }
                Self::Unknown
            },
            sqlx::Error::RowNotFound => Self::ObjDoesNotExists("obj".to_string()),
            _ => Self::Unknown,
        }
    }
}
