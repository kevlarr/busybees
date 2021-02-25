use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use crate::{
    handlers::not_found,
    pages::admin::{PostForm, Posts},
    pages::Page,
    store::{self, posts::{NewPostParams, PostLink}},
    redirect, ActixResult, State,
};

pub async fn get(page: Page, state: Data<State>) -> Page {
    page.id("AdminPosts").title("Manage Posts").content(Posts {
        posts: store::posts::admin_list(&state.pool).await,
    })
}

pub async fn new(page: Page, state: Data<State>) -> ActixResult {
    let author_id = page.user
        .ok_or_else(|| HttpResponse::BadRequest().body("No user present".to_owned()))?
        .id;

    let new_post = NewPostParams {
        author_id,
        title: "New post".into(),
        content: String::new(),
    };

    match store::posts::create(&state.pool, new_post).await {
        Ok(key) => Ok(redirect(&format!("/admin/posts/edit/{}", key))),
        Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
    }
}

pub async fn edit(
    page: Page,
    Path((key,)): Path<(String,)>,
    state: Data<State>,
) -> Page {
    match store::posts::get(&state.pool, key.clone()).await {
        Ok(post) => {
            let href = post.href();

            page.id("PostForm")
                .title("Edit Post")
                .content(PostForm { post })
                .admin_links(vec![
                    (
                        href,
                        "/public/images/file-text.svg".into(),
                        "Preview Post".into(),
                    ), (
                        format!("/admin/posts/delete/{}", key),
                        "/public/images/x-square.svg".into(),
                        "Delete Post".into(),
                    ),
                ])
        },

        Err(e) => {
            eprintln!("{}", e.to_string());
            not_found(page).await
        }
    }
}

//#[deprecated(note = "Don't use GET for delete")]
// FIXME Don't use GET for delete
pub async fn delete(
    Path((key,)): Path<(String,)>,
    state: Data<State>,
) -> ActixResult {
    match store::posts::delete(&state.pool, &key).await {
        Ok(()) => Ok(redirect("/admin/posts")),
        Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
    }
}
