//! Page templates
pub mod admin;

mod about;
mod auth;
mod home;
mod notfound;
mod page;
mod post;
mod sandbox;

pub use page::Page;
pub use about::About;
pub use auth::Auth;
pub use home::Home;
pub use notfound::NotFound;
pub use post::PostView;
pub use sandbox::Sandbox;
