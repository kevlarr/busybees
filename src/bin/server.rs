use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{
    middleware::{DefaultHeaders, Logger},
    web::{get, route, scope},
    App, HttpServer,
};
use busybees::{
    handlers,
    middleware,
    State,
    ASSET_BASEPATH,
};
use std::{env, io};

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

        let file_scope = |url_path, dir_path| scope(url_path)
            .service(file_handler("/", dir_path))
            .wrap(DefaultHeaders::new().header("Cache-Control", "max-age=31536000"));

        // File handlers - I *think* each needs to be scoped to allow for
        // assigning cache headers just on these routes
        let assets = file_scope(&ASSET_BASEPATH, "www/assets");
        let public = file_scope("/public", "www/public");
        let uploads = file_scope("/uploads", &state.upload_path);

        App::new()
            .data(state)

            // First applied is last to execute, so user/session management needs to
            // be applied prior to the cookie session backend
            .wrap_fn(middleware::load_user)
            .wrap_fn(middleware::set_assigns)
            .wrap(cookie_session)
            .wrap(Logger::default())

            // Render "not found" page (200 response)
            .default_service(route().to(handlers::not_found))

            .service(assets)
            .service(public)
            .service(uploads)

            .route("/", get().to(handlers::home))
            .route("/about", get().to(handlers::about))
            .route("/sandbox", get().to(handlers::sandbox))

            .service(handlers::admin::resource("/admin"))
            .service(handlers::api::resource("/api"))
            .service(handlers::auth::resource("/auth"))
            .service(handlers::posts::resource("/posts"))
    };

    HttpServer::new(app)
        .bind(address)?
        .run()
        .await
}

fn file_handler(url_path: &str, dir_path: &str) -> Files {
    Files::new(url_path, dir_path)
        .show_files_listing()
        .use_last_modified(true)
}
