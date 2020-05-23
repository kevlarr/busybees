use crate::pages::admin::auth_guard;

use actix_web::{
    guard::fn_guard,
    web,
    Scope,
};

mod images;
mod posts;

pub fn resource(path: &str) -> Scope {
    use web::{patch, post};

    web::scope(path)
        .guard(fn_guard(auth_guard))
        .route("/posts/{key}", patch().to(posts::update))
        .route("/posts/{key}/images/new", post().to(images::upload))
        .route("/posts/{key}/images/link", post().to(images::link))
        .route("/posts/{key}/published", patch().to(posts::update_published))
}
