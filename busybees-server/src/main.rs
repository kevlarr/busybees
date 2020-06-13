use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{
    middleware::{DefaultHeaders, Logger},
    web::{get, route},
    App, HttpServer,
};
use busybees::{
    deps::{actix_rt, dotenv},
};
use busybees_server::{
    handlers,
    middleware,
    State,
    ASSET_BASEPATH,
};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use std::{env, io};

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let address = env::var("ADDRESS").expect("ADDRESS not set");

    let app = || {
        let upload_path = env::var("UPLOAD_PATH").expect("UPLOAD_PATH not set");
        let state = State::new(upload_path);

        let cookie_session = CookieSession::private(&state.secret_key.as_bytes())
            .name("busybeelife")
            .http_only(true)
            .secure(true);

        let cache_headers = DefaultHeaders::new().header("Cache-Control", "max-age=31536000");

        let assets = file_handler(&ASSET_BASEPATH, "www/assets");
        let public = file_handler("/public", "www/public");
        let uploads = file_handler("/uploads", &state.upload_path);

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

            // File handlers
            .service(assets).wrap(cache_headers.clone())
            .service(public).wrap(cache_headers.clone())
            .service(uploads).wrap(cache_headers)

            .route("/", get().to(handlers::home))
            .route("/about", get().to(handlers::about))
            .route("/sandbox", get().to(handlers::sandbox))

            .service(handlers::admin::resource("/admin"))
            .service(handlers::api::resource("/api"))
            .service(handlers::auth::resource("/auth"))
            .service(handlers::posts::resource("/posts"))
    };

    HttpServer::new(app)
        .bind_openssl(address, ssl_builder())?
        .run()
        .await
}

fn file_handler(url_path: &str, dir_path: &str) -> Files {
    Files::new(url_path, dir_path)
        .show_files_listing()
        .use_last_modified(true)
}

fn ssl_builder() -> SslAcceptorBuilder {
    let key_file = env::var("SSL_KEY_FILE").expect("SSL_KEY_FILE not set");
    let cert_file = env::var("SSL_CERT_FILE").expect("SSL_CERT_FILE not set");

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();

    builder.set_private_key_file(&key_file, SslFiletype::PEM).unwrap();
    builder.set_certificate_chain_file(&cert_file).unwrap();

    builder
}
