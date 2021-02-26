use actix_web::{
    web::Data,
    Either,
    HttpResponse,
};

use crate::{
    pages::{About, Home, NotFound, Page, Sandbox},
    store,
    ActixResult,
    State,
};

pub mod admin;
pub mod api;
pub mod auth;
pub mod posts;

pub async fn home(page: Page, state: Data<State>) -> Either<Page, ActixResult> {
    match store::posts::recent_published(&state.pool).await {
        Ok(previews) => Either::A(
            page.id("Home")
                .title("Latest Posts")
                .content(Home { posts: previews }),
        ),
        Err(_) => Either::B(
            // FIXME This should be an actual page
            Ok(HttpResponse::InternalServerError().finish()),
        ),
    }
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
