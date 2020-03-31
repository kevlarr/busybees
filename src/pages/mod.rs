use actix_web::{HttpResponse, Responder};


pub mod about;
mod auth;
mod index;
mod page;
mod not_found;
mod post;
mod sandbox;

pub use about::AboutPage;
pub use auth::AuthPage;
pub use index::IndexPage;
pub use page::Page;
pub use not_found::NotFoundPage;
pub use post::{PostFormPage, PostPage};
pub use sandbox::SandboxPage;


pub trait Renderable: Into<String> {
    fn render(self) -> HttpResponse {
        HttpResponse::Ok().body::<String>(self.into())
    }
}
