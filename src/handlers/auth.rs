use actix_web::{
    web,
    Error, HttpResponse,
};
use crate::{
    encryption,
    models::Author,
    pages::{AuthPage, Renderable},
    State,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

pub async fn sign_in(
    credentials: web::Form<Credentials>,
    state: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let pool = &mut *state.pool.borrow_mut();
    let secret = &state.secret_key;

    let result = sqlx::query_as!(Author, "
        select email, name, password_hash from author where email = $1
    ", credentials.email)
        .fetch_one(pool)
        .await;

    let author = match result {
        Ok(author) => author,

        // TODO actually match on error type
        Err(_error) => {
            // Hash the password to help prevent timing attacks
            let _ = encryption::hash(secret, &credentials.password);

            // TODO redirect back with error message in cookie instead of rendering here?
            return Ok(AuthPage::with_error("Invalid credentials".into()).render());
        },
    };

    let verified = encryption::verify(secret, &author.password_hash, &credentials.password);

    Ok(match verified {
        Ok(true) => AuthPage::with_error("Not implemented".into()).render(),
        Ok(_) => AuthPage::with_error("Invalid credentials".into()).render(),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    })
}
