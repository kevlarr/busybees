use ::busybees::pages;

use actix_files::Files;
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{
    middleware::Logger,
    web::{self, get, post, route, scope, Path},
    App, Error, HttpResponse, HttpServer, Responder,
};
use chrono::Utc;
use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

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

async fn not_found() -> impl Responder {
    render(pages::NotFound)
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

#[derive(Deserialize, Serialize)]
struct UploadedImages {
    filepaths: Vec<String>,
}

async fn create_post() {

    /*
        id serial primary key,
        title varchar not null,
        body text not null,
        created_at timestamptz not null,
        updated_at timestamptz not null,
    */
}

async fn upload_images(mut payload: Multipart) -> Result<HttpResponse, Error>  {
    let mut filepaths = Vec::new();

    while let Some(item) = payload.next().await {
        let mut field: Field = item?;

        let content_type = field
            .content_disposition()
            .ok_or_else(|| MultipartError::Incomplete)?;

        let filename = content_type
            .get_filename()
            .ok_or_else(|| MultipartError::Incomplete)?;

        let filepath = format!("www/public/uploads/{}", filename);
        filepaths.push(format!("public/uploads/{}", filename));

        // TODO async-std..?
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath)).await?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;

            f = web::block(move || f.write_all(&data).map(|_| f)).await?;
        }
    }

    Ok(HttpResponse::Ok().json(UploadedImages { filepaths }))
}

struct State {
    pool: sqlx::PgPool,
}

impl State {
    fn new() -> Self {
        let pool = sqlx::PgPool::new("postgres://localhost:5432/busybees")
            .now_or_never()
            .unwrap()  // futures Option
            .unwrap(); // sqlx Result

        State { pool }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    pretty_env_logger::init();

    HttpServer::new(|| {
        App::new()
            .data(State::new())
            .wrap(Logger::default())
            .default_service(route().to(not_found))

            .route("/about", get().to(about))
            .route("/images", post().to(upload_images))
            .route("/sandbox", get().to(sandbox))
            .service(
                scope("/posts")
                    .route("/new", get().to(new_post))
                    .route("/{title}", get().to(show_post)),
            )
            .service(
                Files::new("/public", "www/public")
                    .show_files_listing()
                    .use_last_modified(true),
            )
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
