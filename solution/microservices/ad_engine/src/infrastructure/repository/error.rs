#[derive(thiserror::Error, Debug)]
pub enum RepoError {
    #[error("`{0}` does not exists")]
    ObjDoesNotExists(String),

    #[error("invalid unique constraint `{0}`")]
    UniqueConstraint(String),

    #[error("unknown error")]
    Unknown,
}
