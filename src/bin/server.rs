use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{
    middleware::{DefaultHeaders, Logger},
    web::{get, route, scope},
    App, HttpServer,
};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use std::{env, io};

use ::busybees::{
    ASSET_BASEPATH,
    api,
    middleware,
    pages,
    State,
};


#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    pretty_env_logger::init();

    dotenv::dotenv().ok();

    let address = env::var("ADDRESS").expect("ADDRESS not set");

    let ssl_builder = {
        let key_file = env::var("SSL_KEY_FILE").expect("SSL_KEY_FILE not set");
        let cert_file = env::var("SSL_CERT_FILE").expect("SSL_CERT_FILE not set");

        let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
        builder.set_private_key_file(&key_file, SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file(&cert_file).unwrap();

        builder
    };

    HttpServer::new(|| {
        let upload_path = env::var("UPLOAD_PATH").expect("UPLOAD_PATH not set");
        let state = State::new(upload_path);

        let cookie_session = CookieSession::private(&state.secret_key.as_bytes())
            .name("busybeelife")
            .http_only(true)
            .secure(true);

        // TODO Okay, this is some copy/paste...
        let assets = Files::new("/", "www/assets")
            .show_files_listing()
            .use_last_modified(true);

        let static_files = Files::new("/", "www/public")
            .show_files_listing()
            .use_last_modified(true);

        let uploaded_files = Files::new("/", &state.upload_path)
            .show_files_listing()
            .use_last_modified(true);

        App::new()
            .data(state)

            // First applied is last to execute, so user/session management needs to
            // be applied prior to the cookie session backend
            .wrap_fn(middleware::load_user)
            .wrap_fn(middleware::set_assigns)
            .wrap(cookie_session)
            .wrap(Logger::default())

            // Default 404 response
            .default_service(route().to(pages::notfound::get))

            // Public assets and uploaded images
            .service(scope(&ASSET_BASEPATH)
                .service(assets)
                .wrap(DefaultHeaders::new().header("Cache-Control", "max-age=31536000")))

            .service(scope("/public")
                .service(static_files)
                .wrap(DefaultHeaders::new().header("Cache-Control", "max-age=31536000")))

            .service(scope("/uploads")
                .service(uploaded_files)
                .wrap(DefaultHeaders::new().header("Cache-Control", "max-age=31536000")))

            .route("/", get().to(pages::home::get))
            .route("/about", get().to(pages::about::get))
            .route("/sandbox", get().to(pages::sandbox::get))
            .service(pages::auth::resource("/auth"))
            .service(pages::post::resource("/posts"))
            .service(pages::admin::resource("/admin"))
            .service(api::resource("/api"))
    })
    .bind_openssl(address, ssl_builder)?
    .run()
    .await
}
