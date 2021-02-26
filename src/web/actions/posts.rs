use actix_web::{
    web::{Data, Path},
};
use crate::{
    store,
    web::{
        actions::not_found,
        pages::{Page, PostView},
    },
    State,
};

pub async fn get_post(
    page: Page,
    Path((key, _slug)): Path<(String, String)>,
    state: Data<State>,
) -> Page {
    let auth = page.user.is_some();

    match store::posts::get(&state.pool, key.clone()).await {
        Ok(post) => page
            .id("Post")
            .title(post.title.clone())
            .image(post.preview_image_filename.clone())
            .content(PostView { auth, post })
            .admin_links(vec![
                (
                    format!("/admin/posts/edit/{}", key),
                    "/public/images/edit.svg".into(),
                    "Edit Post".into(),
                ), (
                    format!("/admin/posts/delete/{}", key),
                    "/public/images/x-square.svg".into(),
                    "Delete Post".into(),
                ),
            ]),

        Err(_) => not_found(page).await,
    }
}
