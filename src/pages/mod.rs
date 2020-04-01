use actix_web::HttpResponse;

pub mod about;
pub mod auth;
pub mod home;
pub mod notfound;
pub mod posts;
pub mod sandbox;

mod page;

pub use page::Page;


pub trait Renderable: Into<String> {
    fn render(self) -> HttpResponse {
        HttpResponse::Ok().body::<String>(self.into())
    }
}
