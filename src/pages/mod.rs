use actix_web::HttpResponse;

mod about;
mod auth;
mod index;
mod layout;
mod not_found;
mod post;
mod sandbox;

pub use about::AboutPage;
pub use auth::AuthPage;
pub use index::IndexPage;
pub use layout::LayoutPage;
pub use not_found::NotFoundPage;
pub use post::{PostFormPage, PostPage};
pub use sandbox::SandboxPage;

pub trait Renderable: Into<String> {
    fn render(self) -> HttpResponse {
        HttpResponse::Ok().body::<String>(self.into())
    }
}
