use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{
    middleware::Logger,
    web::{self, get, post},
    App, HttpServer,
};
use std::io;

use ::busybees::{
    handlers,
    middleware,
    pages,
    State,
};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    pretty_env_logger::init();

    dotenv::dotenv().ok();

    HttpServer::new(|| {
        let state = State::new();

        let cookie_session = CookieSession::signed(&state.secret_key.as_bytes())
            .name("busybees")
            .secure(false)
            .http_only(false);

        let static_files = Files::new("/public", "www/public")
            .show_files_listing()
            .use_last_modified(true);

        App::new()
            .data(state)

            // First applied is last to execute, so user/session management needs to
            // be applied prior to the cookie session backend
            .wrap(middleware::LoadUser)
            .wrap(middleware::SetAssigns)
            .wrap(cookie_session)
            .wrap(Logger::default())

            // Default 404 response
            .default_service(web::route().to(pages::notfound::get))

            // Public assets
            .service(static_files)

            .route("/", get().to(pages::home::get))
            .route("/about", get().to(pages::about::get))
            .route("/images", post().to(handlers::images::upload))
            .route("/sandbox", get().to(pages::sandbox::get))
            .service(pages::auth::resource("/auth"))
            .service(pages::posts::resource("/posts"))
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
