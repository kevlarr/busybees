use crate::pages::admin::auth_guard;

use actix_web::{
    guard::fn_guard,
    web,
    Scope,
};

mod images;
mod posts;

pub fn resource(path: &str) -> Scope {
    use web::post;

    web::scope(path)
        .guard(fn_guard(auth_guard))
        .route("/images", post().to(images::upload))
        .route("/posts/{key}", post().to(posts::update))
}
