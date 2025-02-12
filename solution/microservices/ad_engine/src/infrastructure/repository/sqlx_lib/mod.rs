mod user_repository;

pub use user_repository::{ClientReturningSchema, PgClientRepository};

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
