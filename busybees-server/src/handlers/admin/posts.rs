use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use busybees::store::{self, posts::PostParams};
use crate::{
    handlers::not_found,
    pages::admin::{PostForm, Posts},
    pages::Page,
    redirect, ActixResult, State,
};

pub async fn get(page: Page, state: Data<State>) -> Page {
    page.id("AdminPosts").title("Manage Posts").content(Posts {
        posts: store::posts::admin_list(&state.pool).await,
    })
}

pub async fn new(state: Data<State>) -> ActixResult {
    let new_post = PostParams {
        title: "New post".into(),
        content: String::new(),
    };

    match store::posts::create(&state.pool, new_post).await {
        Ok(key) => Ok(redirect(&format!("/admin/posts/edit/{}", key))),
        Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
    }
}

pub async fn edit(page: Page, path: Path<(String,)>, state: Data<State>) -> Page {
    match store::posts::get(&state.pool, path.0.clone()).await {
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

pub async fn delete(path: Path<(String,)>, state: Data<State>) -> ActixResult {
    match store::posts::delete(&state.pool, &path.0).await {
        Ok(()) => Ok(redirect("/admin/posts")),
        Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
    }
}
