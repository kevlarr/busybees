use actix_web::{
    Either, HttpResponse,
    dev::RequestHead,
    web::Data,
};
use crate::{ActixResult, State};
use crate::extensions::Assigns;
use crate::pages::{About, Home, NotFound, Page, Sandbox};
use crate::store::{self, authors::AuthorWithoutPassword};

pub mod admin;
pub mod api;
pub mod auth;
pub mod posts;

pub fn auth_guard(head: &RequestHead) -> bool {
    let author: Option<AuthorWithoutPassword> = head
        .extensions()
        .get::<Assigns>()
        .map(|assn| assn.author.clone())
        .flatten();

    author.is_some()
}

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
