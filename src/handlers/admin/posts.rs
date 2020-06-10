use actix_web::{
    web::{Data, Path},
    HttpResponse,
};

use crate::handlers::not_found;
use crate::pages::admin::{PostForm, Posts};
use crate::pages::Page;
use crate::store::posts::{self, PostParams};
use crate::{redirect, ActixResult, State};

pub async fn get(page: Page, state: Data<State>) -> Page {
    page.id("AdminPosts").title("Manage Posts").content(Posts {
        posts: posts::admin_list(&state.pool).await,
    })
}

pub async fn delete(path: Path<(String,)>, state: Data<State>) -> ActixResult {
    match posts::delete(&state.pool, &path.0).await {
        Ok(()) => Ok(redirect("/admin/posts")),
        Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
    }
}

pub async fn new(state: Data<State>) -> ActixResult {
    let new_post = PostParams {
        title: "New post".into(),
        content: String::new(),
    };

    match posts::create(&state.pool, new_post).await {
        Ok(key) => Ok(redirect(&format!("/admin/posts/edit/{}", key))),
        Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
    }
}

pub async fn edit(page: Page, path: Path<(String,)>, state: Data<State>) -> Page {
    match posts::find(&state.pool, path.0.clone()).await {
        Ok(post) => page
            .id("PostForm")
            .title("Edit Post")
            .content(PostForm { post }),

        Err(e) => {
            eprintln!("{}", e.to_string());
            not_found(page).await
        }
    }
}
