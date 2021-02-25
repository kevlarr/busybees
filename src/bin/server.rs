use std::{env, io};

use actix_session::CookieSession;
use actix_web::{
    middleware::{DefaultHeaders, Logger},
    App, HttpServer,
};

use busybees::{
    actions as ax,
    guards::auth_guard,
    middleware,
    routes::Routes,
    ASSET_BASEPATH,
    State,
};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let address = env::var("ADDRESS").expect("ADDRESS not set");

    let app = || {
        let state = State::new();
        let cookie_session = CookieSession::private(&state.secret_key.as_bytes())
            .name("busybeelife")
            .http_only(true)
            .secure(true);

        let routes = Routes::new("/")
            .map_files(
                &ASSET_BASEPATH,
                "www/assets",
                |files| files
                    .wrap(DefaultHeaders::new().header("Cache-Control", "max-age=31536000"))
            )
            .map_files(
                "/public",
                "www/public",
                |files| files
                    .wrap(DefaultHeaders::new().header("Cache-Control", "max-age=31536000"))
            )
            .map_files(
                "/uploads",
                &state.upload_path,
                |files| files
                    .wrap(DefaultHeaders::new().header("Cache-Control", "max-age=31536000"))
            )
            .get("", ax::home)
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
            .default(ax::not_found);

        App::new()
            .data(state)

            // First applied is last to execute, so user/session management needs to
            // be applied prior to the cookie session backend
            .wrap_fn(middleware::load_user)
            .wrap_fn(middleware::set_assigns)
            .wrap(cookie_session)
            .wrap(Logger::default())

            .service(routes.service)
    };

    HttpServer::new(app)
        .bind(address)?
        .run()
        .await
}
