use actix_web::{
    web::{self, Data, Path},
    Scope,
};
use crate::{
    handlers::not_found,
    pages::{Page, PostView},
    store,
    State,
};

pub fn resource(path: &str) -> Scope {
    web::scope(path).route("/{key}/read/{slug}", web::get().to(get_post))
}

pub async fn get_post(page: Page, path: Path<(String, String)>, state: Data<State>) -> Page {
    let auth = page.user.is_some();

    match store::posts::get(&state.pool, path.0.clone()).await {
        Ok(post) => page
            .id("Post")
            .title(post.title.clone())
            .image(post.preview_image_filename.clone())
            .content(PostView { auth, post })
            .admin_links(vec![
                (
                    format!("/admin/posts/edit/{}", path.0),
                    "/public/images/edit.svg".into(),
                    "Edit Post".into(),
                ), (
                    format!("/admin/posts/delete/{}", path.0),
                    "/public/images/x-square.svg".into(),
                    "Delete Post".into(),
                ),
            ]),

        Err(_) => not_found(page).await,
    }
}
