use sqlx::Error as SqlxError;

pub mod authors;
pub mod images;
pub mod posts;

pub type StoreResult<T> = Result<T, SqlxError>;
