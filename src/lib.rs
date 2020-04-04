use actix_web::{http::header, Error, HttpResponse};

pub mod api;
pub mod encryption;
pub mod extensions;
pub mod middleware;
pub mod models;
pub mod pages;

mod state;
pub use state::State;

pub type ActixResult = Result<HttpResponse, Error>;

pub fn redirect(path: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(header::LOCATION, path)
        .finish()
        .into_body()
}
