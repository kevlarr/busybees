use actix_web::guard::fn_guard;
use actix_web::{web, Scope};

use crate::handlers::auth_guard;

mod posts;

#[deprecated(note = "don't use `GET` for delete")]
pub fn resource(path: &str) -> Scope {
    use web::get;

    web::scope(path)
        .guard(fn_guard(auth_guard))
        .route("/posts", get().to(posts::get))
        .route("/posts/new", get().to(posts::new))
        .route("/posts/edit/{key}", get().to(posts::edit))
        // TODO GET for DELETE is a little weird
        .route("/posts/delete/{key}", get().to(posts::delete))
}
