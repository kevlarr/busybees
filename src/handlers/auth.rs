use crate::{
    encryption,
    models::Author,
    pages::{AuthPage, Page, Renderable},
    State,
};
use actix_session::Session;
use actix_web::{http, web, Either, Error, HttpResponse, Responder};
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
    page: Page,
) -> Either<Result<HttpResponse, Error>, impl Responder> {
    let pool = &mut *state.pool.borrow_mut();
    let secret = &state.secret_key;

    let result = sqlx::query_as!(
        Author,
        "select id, email, name, password_hash from author where email = $1",
        credentials.email
    ).fetch_one(pool).await;

    let author = match result {
        Ok(author) => author,

        Err(_) => {
            // Hash the password anyway to help prevent timing attacks
            let _ = encryption::hash(secret, &credentials.password);
            return Either::B(page.with_content(AuthPage::with_error("Invalid credentials".into())));
        }
    };

    let verified = encryption::verify(secret, &author.password_hash, &credentials.password);

    match verified {
        Ok(true) => match session.set("auth", author.id) {
            Ok(_) => Either::A(Ok(HttpResponse::Found()
                .header(http::header::LOCATION, "/")
                .finish()
                .into_body())),

            Err(e) => Either::B(page.with_content(AuthPage::with_error(e.to_string()))),
        },
        Ok(_) => Either::B(page.with_content(AuthPage::with_error("Invalid credentials".into()))),
        Err(e) => Either::B(page.with_content(AuthPage::with_error(e.into()))),
    }
}
