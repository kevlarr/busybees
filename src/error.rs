//! Error module
use actix_multipart::MultipartError;
use actix_web::error::{Error as AwError, ResponseError};
use actix_web::HttpResponse;
use crate::{
    encryption::EncryptionError,
    imaging::ImagingError,
};
use regex::Error as RegexError;
use sqlx::Error as SqlxError;
use std::fmt;

/// Enum of all errors that should be convertable to an HTTP response
/// and returned directly to the client.
#[derive(Debug)]
pub enum ApiError {
    ActixWeb(AwError),
    ActixMultipart(MultipartError),
    Database(SqlxError),
    Encryption(EncryptionError),
    Imaging(ImagingError),
    ServerError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        use ApiError::*;

        eprintln!("{}", self);

        match self {
            Database(e) => match e {
                SqlxError::RowNotFound => HttpResponse::NotFound().finish(),
                _ => HttpResponse::InternalServerError().finish(),
            },

            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<AwError> for ApiError {
    fn from(err: AwError) -> Self {
        ApiError::ActixWeb(err)
    }
}

impl From<MultipartError> for ApiError {
    fn from(err: MultipartError) -> Self {
        ApiError::ActixMultipart(err)
    }
}

impl From<EncryptionError> for ApiError {
    fn from(err: EncryptionError) -> Self {
        ApiError::Encryption(err)
    }
}

impl From<ImagingError> for ApiError {
    fn from(err: ImagingError) -> Self {
        ApiError::Imaging(err)
    }
}

impl From<RegexError> for ApiError {
    fn from(err: RegexError) -> Self {
        ApiError::ServerError(err.to_string())
    }
}

impl From<SqlxError> for ApiError {
    fn from(err: SqlxError) -> Self {
        ApiError::Database(err)
    }
}
