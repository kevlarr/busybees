use actix_web::{
    web::{self, Data, Path},
    Scope,
};
use busybees::store;
use crate::{
    handlers::not_found,
    pages::{Page, PostView},
    State,
};

pub fn resource(path: &str) -> Scope {
    web::scope(path).route("/{key}/read/{slug}", web::get().to(get_post))
}

pub async fn get_post(page: Page, path: Path<(String, String)>, state: Data<State>) -> Page {
    let auth = page.user.is_some();

    match store::posts::find(&state.pool, path.0.clone()).await {
        Ok(post) => page
            .id("Post")
            .title(post.title.clone())
            .image(post.thumbnail.clone())
            .content(PostView { auth, post }),

        Err(_) => not_found(page).await,
    }
}