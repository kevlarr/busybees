use crate::{
    encryption,
    models::Author,
    pages::Page,
    State,
};
use actix_session::Session;
use actix_web::{
    http::header::LOCATION,
    web::{self, Data, Form},
    Either,
    Error,
    HttpResponse,
    Resource,
};
use horrorshow::{html, RenderOnce, TemplateBuffer};
use serde::Deserialize;


pub fn resource(path: &str) -> Resource {
    web::resource(path)
        .route(web::get().to(Auth::get))
        .route(web::post().to(Auth::post))
}


#[derive(Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}


pub struct Auth {
    error_message: Option<String>,
}


impl Auth {
    pub fn new() -> Self {
        Auth {
            error_message: None,
        }
    }

    pub fn with_error(msg: impl Into<String>) -> Self {
        Auth {
            error_message: Some(msg.into()),
        }
    }

    fn in_page(self, page: Page) -> Page {
        page.title("Sign In")
            .id("Auth")
            .content(self)
    }

    pub async fn get(page: Page) -> Page {
        Self::new().in_page(page)
    }

    pub async fn post(
        credentials: Form<Credentials>,
        state: Data<State>,
        session: Session,
        page: Page,
    ) -> Either<Result<HttpResponse, Error>, Page> {
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

                return Either::B(Auth::with_error("Invalid credentials").in_page(page));
            }
        };

        Either::B(
            match encryption::verify(secret, &author.password_hash, &credentials.password) {
                Ok(true) => match session.set("auth", author.id) {
                    Ok(_) => return Either::A(Ok(HttpResponse::Found()
                        .header(LOCATION, "/")
                        .finish()
                        .into_body())),

                    Err(e) => Auth::with_error(e.to_string()).in_page(page),
                },
                Ok(_) => Auth::with_error("Invalid credentials").in_page(page),
                Err(e) => Auth::with_error(e.to_string()).in_page(page),
            }
        )
    }
}


impl RenderOnce for Auth {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Auth { error_message } = self;

        tmpl << html! {
            form (method = "post", action = "/auth") {
                h1 (id = "SignInWelcome") : "W";

                input (id = "SignInEmail",    name = "email",    type = "email",    placeholder = "Email", autofocus);
                input (id = "SignInPassword", name = "password", type = "password", placeholder = "Password");

                @ if let Some(msg) = error_message {
                    form-message (type = "error") : msg;
                }

                button (id = "SignInSubmit",  type = "submit", class = "primary", disabled) : "Sign In";
            }

            script(src = "/public/assets/signin.js");
        };
    }
}
