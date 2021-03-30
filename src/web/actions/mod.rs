use actix_web::{
    web::Data,
    Either,
    HttpResponse,
};

use crate::{
    store::{self, posts::PublishedPostMeta},
    web::pages::{About, NotFound, Page, Sandbox},
    ActixResult,
    State,
};

pub mod admin;
pub mod api;
pub mod auth;
pub mod posts;

//pub async fn home(page: Page, state: Data<State>) -> Either<Page, ActixResult> {
pub async fn root(state: Data<State>) -> Result<Vec<PublishedPostMeta>, String> {
    store::posts::recent_published(&state.pool).await
        .map_err(|e| e.to_string())
}

pub async fn about(page: Page) -> Page {
    page.id("About").title("About Us").content(About {})
}

pub async fn not_found(page: Page) -> Page {
    page.id("NotFound").title("Not Found").content(NotFound {})
}

pub async fn sandbox(page: Page) -> Page {
    page.id("Sandbox").title("Sandbox").content(Sandbox {})
}
