
use actix_files::Files;
use actix_web::{
    guard::fn_guard,
    middleware::DefaultHeaders,
    web::{self, route, scope},
    Scope,
};

use crate::{
    actions::*,
    guards::auth_guard,
    ASSET_BASEPATH,
    State,
};


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
