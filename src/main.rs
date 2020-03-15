use ::busybees::pages;

use actix_files::Files;
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{
    http,
    middleware::Logger,
    web::{self, get, post, Path},
    App, Error, HttpResponse, HttpServer, Responder,
};
use chrono::Utc;
use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, io::{self, Write}, rc::Rc};


struct State {
    pool: Rc<RefCell<sqlx::PgPool>>,
}

impl State {
    fn new() -> Self {
        let pool = sqlx::PgPool::new("postgres://localhost:5432/busybees")
            .now_or_never()
            .unwrap()  // futures Option
            .unwrap(); // sqlx Result

        State { pool: Rc::new(RefCell::new(pool)) }
    }
}

#[derive(Serialize)]
struct UploadedImages {
    filepaths: Vec<String>,
}


#[derive(Debug, Deserialize)]
struct NewPostParams {
    alpha_id: String,
    title: String,
    content: String,
}


// TODO these are *begging* for a trait
fn render(s: impl Into<String>) -> HttpResponse {
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

async fn show_post(
    path: Path<(String,)>,
    state: web::Data<State>
) -> Result<HttpResponse, Error>  {
    let pool = &mut *state.pool.borrow_mut();
    let alpha_id = match path.0.splitn(2, '-').next() {
        Some(aid) => aid,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    let result = sqlx::query!("
        select title, content, published, created_at, updated_at
        from posts where alphanumeric_id = $1
    ", alpha_id.to_string())
        .fetch_one(pool)
        .await;

    match result {
        Ok(row) => {
            let post = pages::Post {
                content: row.content.clone(),
                published: row.published,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };

            Ok(render(post))
        },
        Err(_) => Ok(HttpResponse::NotFound().finish()),
    }
}

async fn sandbox() -> impl Responder {
    render(pages::Sandbox)
}

async fn create_post(
    form: web::Form<NewPostParams>,
    state: web::Data<State>
) -> Result<HttpResponse, Error>  {
    let pool = &mut *state.pool.borrow_mut();

    let now = Utc::now();
    let result = sqlx::query!("
        insert into posts
        (alphanumeric_id, title, content, published, created_at, updated_at)
            values ($1, $2, $3, $4, $5, $6)
    ", form.alpha_id, form.title, form.content, false, now, now)
        .execute(pool)
        .await;

    if let Err(e) = result {
        return Ok(HttpResponse::BadRequest().body(e.to_string()));
    }

    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, format!("/posts/{}-{}", form.alpha_id, form.title))
        .finish()
        .into_body())
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

#[actix_rt::main]
async fn main() -> io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    pretty_env_logger::init();

    HttpServer::new(|| {
        App::new()
            .data(State::new())
            .wrap(Logger::default())
            .default_service(web::route().to(not_found))

            .route("/about", get().to(about))
            .route("/images", post().to(upload_images))
            .route("/sandbox", get().to(sandbox))
            .service(
                web::scope("/posts")
                    .route("/new", get().to(new_post))
                    .route("/new", post().to(create_post))
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
