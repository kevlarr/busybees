use ::busybees::pages;
use actix_web::{
    web::{get, post, scope, Path},
    App, HttpResponse, HttpServer, Responder,
};
use chrono::Utc;

// TODO these are *begging* for a trait
fn render(s: impl Into<String>) -> impl Responder {
    HttpResponse::Ok().body::<String>(s.into())
}

async fn about() -> impl Responder {
    render(pages::About)
}

async fn new_post() -> impl Responder {
    render(pages::NewPost)
}

async fn show_post(path: Path<(String,)>) -> impl Responder {
    let now = Utc::now();
    let post = pages::Post {
        title: path.0.clone(),
        body: "<p style='color: red'>some content</p>".into(),
        created_at: now.clone(),
        updated_at: now,
    };

    render(post)
}

async fn sandbox() -> impl Responder {
    render(pages::Sandbox)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/about", get().to(about))
            .route("/sandbox", get().to(sandbox))
            .service(
                scope("/posts")
                    .route("/new", get().to(new_post))
                    .route("/{title}", get().to(show_post)),
            )
            .service(
                actix_files::Files::new("/public", "www/public")
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
