use actix_web::{http::header, Error as ActixError, HttpResponse};
use chrono::Utc;
use lazy_static::lazy_static;

pub mod encryption;
pub mod error;
pub mod extensions;
pub mod handlers;
pub mod imaging;
pub mod middleware;
pub mod state;
pub mod store;
pub mod pages;

pub use error::ApiError;
pub use state::State;

/// Simple result type using raw `actix_web` types.
pub type ActixResult = Result<HttpResponse, ActixError>;

/// Generic result type based on `crate::error::ApiError` variants
/// that can directly be converted to `HttpResponse`.
pub type ApiResult<T> = Result<T, ApiError>;


lazy_static! {
    /// The dynamic static asset path, generated at server start-up,
    /// that enables cache-busting without changing filenames.
    pub static ref ASSET_BASEPATH: String = format!("/assets/{}", Utc::now().timestamp());
}

/// Helper to generate the dynamic asset path for the given src name.
pub fn asset_path(filename: &str) -> String {
    format!("{}/{}", *ASSET_BASEPATH, filename)
}

/// Helper to create redirect responses
pub fn redirect(path: &str) -> HttpResponse {
    HttpResponse::Found().header(header::LOCATION, path).finish()
}
