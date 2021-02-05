use actix_web::{guard::fn_guard, web, Scope};
use crate::handlers::auth_guard;

mod posts;

pub fn resource(path: &str) -> Scope {
    use web::{get, patch, post};

    web::scope(path)
        .guard(fn_guard(auth_guard))
        .route("/posts/{key}", patch().to(posts::update))
        .route("/posts/{key}/images", get().to(posts::images::list))
        .route("/posts/{key}/images/new", post().to(posts::images::upload))
        .route("/posts/{key}/published", patch().to(posts::update_published))
}
