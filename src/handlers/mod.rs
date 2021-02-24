use actix_files::Files;
use actix_web::{
    dev::RequestHead,
    middleware::DefaultHeaders,
    web::{self, Data},
    Either,
    HttpResponse,
    Scope,
};

use crate::{
    extensions::Assigns,
    pages::{About, Home, NotFound, Page, Sandbox},
    store::{self, authors::AuthorWithoutPassword},
    ASSET_BASEPATH,
    ActixResult,
    State,
};


pub mod admin;
pub mod api;
pub mod auth;
pub mod posts;



fn file_handler(url_path: &str, dir_path: &str) -> Files {
    Files::new(url_path, dir_path)
        .show_files_listing()
        .use_last_modified(true)
}

pub fn router(state: &State) -> Scope {
    use web::get;

    let file_scope = |url_path, dir_path| web::scope(url_path)
        .service(file_handler("/", dir_path))
        .wrap(DefaultHeaders::new().header("Cache-Control", "max-age=31536000"));

    // File handlers - I *think* each needs to be scoped to allow for
    // assigning cache headers just on these routes
    let assets = file_scope(&ASSET_BASEPATH, "www/assets");
    let public = file_scope("/public", "www/public");
    let uploads = file_scope("/uploads", &state.upload_path);

    web::scope("/")
        .route("/", get().to(home))
        .route("/about", get().to(about))
        .route("/sandbox", get().to(sandbox))
        .service(admin::resource("/admin"))
        .service(api::resource("/api"))
        .service(auth::resource("/auth"))
        .service(posts::resource("/posts"))
        .service(assets)
        .service(public)
        .service(uploads)
}

pub fn auth_guard(head: &RequestHead) -> bool {
    let author: Option<AuthorWithoutPassword> = head
        .extensions()
        .get::<Assigns>()
        .map(|assn| assn.author.clone())
        .flatten();

    author.is_some()
}

async fn home(page: Page, state: Data<State>) -> Either<Page, ActixResult> {
    match store::posts::recent_published(&state.pool).await {
        Ok(previews) => Either::A(
            page.id("Home")
                .title("Latest Posts")
                .content(Home { posts: previews }),
        ),
        Err(_) => Either::B(
            // FIXME This should be an actual page
            Ok(HttpResponse::InternalServerError().finish()),
        ),
    }
}

async fn about(page: Page) -> Page {
    page.id("About").title("About Us").content(About {})
}

pub async fn not_found(page: Page) -> Page {
    page.id("NotFound").title("Not Found").content(NotFound {})
}

async fn sandbox(page: Page) -> Page {
    page.id("Sandbox").title("Sandbox").content(Sandbox {})
}
