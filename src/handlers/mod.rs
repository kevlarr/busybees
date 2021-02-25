use actix_files::Files;
use actix_web::{
    dev::RequestHead,
    guard::fn_guard,
    middleware::DefaultHeaders,
    web::{self, Data, route, scope},
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

pub fn routes(state: &State) -> Scope {
    use web::{get, patch, post};

    let file_scope = |url_path, dir_path| web::scope(url_path)
        .service(file_handler("/", dir_path))
        .wrap(DefaultHeaders::new().header("Cache-Control", "max-age=31536000"));

    // File handlers - I *think* each needs to be scoped to allow for
    // assigning cache headers just on these routes
    let assets = file_scope(&ASSET_BASEPATH, "www/assets");
    let public = file_scope("/public", "www/public");
    let uploads = file_scope("/uploads", &state.upload_path);

    let guard = || fn_guard(auth_guard);

    scope("/")
        // Render "not found" page (200 response)
        .default_service(route().to(not_found))

        // File paths
        .service(assets)
        .service(public)
        .service(uploads)

        .route("", get().to(home))
        .route("/about", get().to(about))
        .route("/sandbox", get().to(sandbox))

        .service(scope("/admin")
            .guard(guard())
            .service(scope("/posts")
                .route("/", get().to(admin::posts::get))
                .route("/new", get().to(admin::posts::new))
                .route("/edit/{key}", get().to(admin::posts::edit))
                .route("/delete/{key}", get().to(admin::posts::delete))
            )
        )
        .service(scope("/api")
            .guard(guard())
            .service(scope("/posts")
                .service(scope("/{key}")
                    .route("/", patch().to(api::posts::update))
                    .route("/images", get().to(api::posts::images::list))
                    .route("/images/new", post().to(api::posts::images::upload))
                    .route("/published", patch().to(api::posts::update_published))
                )
            )
        )
        .service(scope("/auth")
            .route("", web::get().to(auth::get))
            .route("", web::post().to(auth::post))
            .route("/clear", web::get().to(auth::delete))
        )
        .service(scope("/posts")
            .route("/{key}/read/{slug}", get().to(posts::get_post))
        )
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
