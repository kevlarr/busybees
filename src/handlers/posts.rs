use actix_web::{
    web::{self, Data, Path},
    Scope,
};

use crate::{
    handlers::not_found,
    pages::{Page, PostView},
    store::posts,
    State,
};

pub fn resource(path: &str) -> Scope {
    web::scope(path).route("/{key}/read/{slug}", web::get().to(get_post))
}

pub async fn get_post(page: Page, path: Path<(String, String)>, state: Data<State>) -> Page {
    let auth = page.user.is_some();

    match posts::find(&state.pool, path.0.clone()).await {
        Ok(post) => page
            .id("Post")
            .title(post.title.clone())
            .image(post.first_image.clone())
            .content(PostView { auth, post }),

        Err(_) => not_found(page).await,
    }
}
