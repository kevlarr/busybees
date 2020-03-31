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
    pages::{self, Renderable},
    State,
};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    pretty_env_logger::init();

    dotenv::dotenv().ok();

    HttpServer::new(|| {
        let state = State::new();

        App::new()
            .wrap(Logger::default())
            .wrap(
                CookieSession::private(&state.secret_key.as_bytes())
                    .name("busybees")
                    .secure(false),
            )
            .data(state)
            .default_service(web::route().to(|| pages::NotFoundPage {}.render()))
            .route("/", get().to(handlers::posts::index))
            .route("/about", get().to(|| pages::AboutPage {}.render()))
            .route("/images", post().to(handlers::images::upload))
            .route("/sandbox", get().to(|| pages::SandboxPage {}.render()))
            .service(
                web::resource("/auth")
                    .route(get().to(|| pages::AuthPage::new().render()))
                    .route(post().to(handlers::auth::sign_in)),
            )
            .service(
                web::scope("/posts")
                    .route(
                        "/new",
                        get().to(|| pages::PostFormPage { post: None }.render()),
                    )
                    .route("/new", post().to(handlers::posts::create))
                    .route("/{key}/edit", get().to(handlers::posts::edit))
                    .route("/{key}/edit", post().to(handlers::posts::update))
                    .route("/{key}/read/{slug}", get().to(handlers::posts::read)),
            )
            .service(
                Files::new("/public", "www/public")
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}