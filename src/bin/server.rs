use std::{env, io};

use actix_session::CookieSession;
use actix_web::{
    middleware::Logger,
    App, HttpServer,
};

use busybees::{
    middleware,
    routes,
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

        let routes_ = routes::routes(&state);

        App::new()
            .data(state)

            // First applied is last to execute, so user/session management needs to
            // be applied prior to the cookie session backend
            .wrap_fn(middleware::load_user)
            .wrap_fn(middleware::set_assigns)
            .wrap(cookie_session)
            .wrap(Logger::default())

            .service(routes_)
    };

    HttpServer::new(app)
        .bind(address)?
        .run()
        .await
}
