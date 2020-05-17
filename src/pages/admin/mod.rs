use crate::{
    extensions::Assigns,
    models::AuthorWithoutPassword,
};

use actix_web::{
    dev::RequestHead,
    guard::fn_guard,
    web,
    Scope,
};

mod posts;
mod form;

use posts::Posts;
use form::PostForm;


pub fn auth_guard(head: &RequestHead) -> bool {
    let author: Option<AuthorWithoutPassword> = head
        .extensions()
        .get::<Assigns>()
        .map(|assn| assn.author.clone())
        .flatten();

    author.is_some()
}


pub fn resource(path: &str) -> Scope {
    use web::{get, post};

    web::scope(path)
        .guard(fn_guard(auth_guard))
        .route("/posts", get().to(Posts::get))
        .route("/posts/new", get().to(PostForm::new))
        .route("/posts/edit/{key}", get().to(PostForm::edit))
        .route("/posts/edit/{key}", post().to(PostForm::update))

        // TODO GET for DELETE is a little weird
        .route("/posts/delete/{key}", get().to(Posts::delete))
}
