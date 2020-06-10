use actix_session::Session;
use actix_web::{
    http::header::LOCATION,
    web::{self, Data, Form},
    Either,
    Error,
    HttpResponse,
    Scope,
};
use serde::Deserialize;

use crate::{
    encryption,
    store::authors::{self, AuthorWithoutPassword},
    pages::{Page, Auth},
    ActixResult,
    State,
    redirect,
};

#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

pub fn resource(path: &str) -> Scope {
    web::scope(path)
        .route("", web::get().to(get))
        .route("", web::post().to(post))
        .route("/clear", web::get().to(delete))
}

pub async fn get(page: Page) -> Page {
    Auth::new().in_page(page)
}

pub async fn post(
    credentials: Form<Credentials>,
    state: Data<State>,
    session: Session,
    page: Page,
) -> Either<Result<HttpResponse, Error>, Page> {
    let secret = &state.secret_key;

    let author = match authors::find(&state.pool, credentials.email.clone()).await {
        Ok(author) => author,
        Err(_) => {
            // Hash the password anyway to help prevent timing attacks
            let _ = encryption::hash(secret, &credentials.password);
            return Either::B(Auth::with_error("Invalid credentials").in_page(page));
        }
    };

    Either::B(
        match encryption::verify(secret, &author.password_hash, &credentials.password) {
            Ok(true) => match session.set::<AuthorWithoutPassword>("auth", author.into()) {
                Ok(_) => return Either::A(Ok(
                    HttpResponse::Found().header(LOCATION, "/admin/posts").finish().into_body()
                )),
                Err(e) => Auth::with_error(e.to_string()).in_page(page),
            },
            Ok(_) => Auth::with_error("Invalid credentials").in_page(page),
            Err(e) => Auth::with_error(e.to_string()).in_page(page),
        }
    )
}

pub async fn delete(session: Session) -> ActixResult {
    session.remove("auth");
    Ok(redirect("/"))
}
