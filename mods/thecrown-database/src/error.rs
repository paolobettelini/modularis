use thiserror::Error;

pub type Result<T> = std::result::Result<T, DatabaseError>;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("database pool error: {0}")]
    Pool(#[from] diesel::r2d2::PoolError),

    #[error("database query error: {0}")]
    Query(#[from] diesel::result::Error),

    #[error("database migration error: {0}")]
    Migration(String),

    #[error("invalid UUID stored in database: {0}")]
    InvalidUuid(#[from] uuid::Error),
}
