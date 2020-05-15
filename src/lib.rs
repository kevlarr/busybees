use actix_web::{http::header, Error, HttpResponse};
use chrono::Utc;
use lazy_static::lazy_static;

pub mod api;
pub mod encryption;
pub mod extensions;
pub mod imaging;
pub mod middleware;
pub mod models;
pub mod pages;

mod state;
pub use state::State;

pub type ActixResult = Result<HttpResponse, Error>;


lazy_static! {
    pub static ref ASSET_BASEPATH: String = format!("/assets/{}", Utc::now().timestamp());
}

pub fn asset_path(filename: &str) -> String {
    format!("{}/{}", *ASSET_BASEPATH, filename)
}


pub fn redirect(path: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(header::LOCATION, path)
        .finish()
        .into_body()
}
