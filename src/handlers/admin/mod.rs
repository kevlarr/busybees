use actix_web::guard::fn_guard;
use actix_web::{web, Scope};

use crate::handlers::auth_guard;

mod posts;


pub fn resource(path: &str) -> Scope {
    web::scope(path)
        .guard(fn_guard(auth_guard))
        .service(posts_resource("/posts"))
}


#[deprecated(note = "don't use `GET` for delete")]
fn posts_resource(path: &str) -> Scope {
    use web::get;

    web::scope(path)
        .route("/", get().to(posts::get))
        .route("/new", get().to(posts::new))
        .route("/edit/{key}", get().to(posts::edit))
        // TODO GET for DELETE is a little weird
        .route("/delete/{key}", get().to(posts::delete))
}
