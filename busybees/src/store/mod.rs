//! Models and utilities for interesting with the data store
use sqlx::Error as SqlxError;

pub mod authors;
pub mod images;
pub mod posts;

pub type StoreResult<T> = Result<T, SqlxError>;
