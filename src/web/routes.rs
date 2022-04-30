use actix_web::{
    middleware::DefaultHeaders,
    Scope,
};

use crate::{
    router::Router,
    web::{
        actions as ax,
        guards::auth_guard,
        views as vw,
    },
    ASSET_BASEPATH,
    State,
};

pub fn service(state: &State) -> Scope {
    let cache_headers = DefaultHeaders::new().header("Cache-Control", "max-age=31536000");
    
    Router::new("/")
        .default(ax::not_found)
        /*.get("", ax::home)*/
        .get("/about", ax::about)
        .get("/sandbox", ax::sandbox)
        .scope("/admin", |admin| admin
            .guard(auth_guard)
            .scope("/posts", |posts| posts
                .get("/", ax::admin::posts::get)
                .get("/new", ax::admin::posts::new)
                .get("/edit/{key}", ax::admin::posts::edit)
                .get("/delete/{key}", ax::admin::posts::delete)
            )
            .get("/test", ax::about)
        )
        .scope("/api", |api| api
            .guard(auth_guard)
            .scope("/posts", |posts| posts
                .scope("/{key}", |post| post
                    .patch("/", ax::api::posts::update)
                    .get  ("/images", ax::api::posts::images::list)
                    .post ("/images/new", ax::api::posts::images::upload)
                    .patch("/published", ax::api::posts::update_published)
                )
            )
        )
        .scope("/auth", |auth| auth
            .get("", ax::auth::get)
            .post("", ax::auth::post)
            .get("/clear", ax::auth::delete)
        )
        .scope("/posts", |posts| posts
            .get("/{key}/read/{slug}", ax::posts::get_post)
        )
        .map_files(
            &ASSET_BASEPATH,
            "www/assets",
            |files| files .wrap(cache_headers.clone()),
        )
        .map_files(
            "/public",
            "www/public",
            |files| files .wrap(cache_headers.clone()),
        )
        .map_files(
            "/uploads",
            &state.upload_path,
            |files| files .wrap(cache_headers.clone()),
        )
        .service
}
