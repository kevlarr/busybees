use crate::{
    encryption,
    models::Author,
    pages::{AuthPage, Renderable},
    State,
};
use actix_session::Session;
use actix_web::{http, web, Error, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

pub async fn sign_in(
    credentials: web::Form<Credentials>,
    state: web::Data<State>,
    session: Session,
) -> Result<HttpResponse, Error> {
    let pool = &mut *state.pool.borrow_mut();
    let secret = &state.secret_key;

    let result = sqlx::query_as!(
        Author,
        "
        select id, email, name, password_hash from author where email = $1
    ",
        credentials.email
    )
    .fetch_one(pool)
    .await;

    let author = match result {
        Ok(author) => author,

        // TODO actually match on error type
        Err(_error) => {
            // Hash the password to help prevent timing attacks
            // TODO How different is the timing on this vs verify below?
            let _ = encryption::hash(secret, &credentials.password);

            return Ok(AuthPage::with_error("Invalid credentials".into()).render());
        }
    };

    let verified = encryption::verify(secret, &author.password_hash, &credentials.password);

    Ok(match verified {
        Ok(true) => match session.set("auth", author.id) {
            Ok(_) => HttpResponse::Found()
                .header(http::header::LOCATION, "/")
                .finish()
                .into_body(),

            Err(e) => AuthPage::with_error(e.to_string()).render(),
        },
        Ok(_) => AuthPage::with_error("Invalid credentials".into()).render(),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    })
}
