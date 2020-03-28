use crate::{
    models::{Post, PostParams, PostPreview},
    pages::{self, Renderable},
    State,
};
use actix_web::{
    http,
    web::{self, Path},
    Error, HttpResponse,
};
use chrono::Utc;
use sqlx::PgPool;

pub async fn index(state: web::Data<State>) -> Result<HttpResponse, Error> {
    let pool = &mut *state.pool.borrow_mut();

    let result = sqlx::query_as!(
        PostPreview,
        r#"select key, title, created_at, substring(content, 'src="([a-zA-Z0-9\.\-_~:\/%\?#=]+)"') as first_src
            from post order by created_at desc limit 4"#,
    )
        .fetch_all(pool)
        .await;

    match result {
        Ok(posts) => Ok(pages::IndexPage { posts }.render()),
        Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
    }
}

pub async fn read(
    path: Path<(String, String)>,
    state: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let pool = &mut *state.pool.borrow_mut();

    match load_post(pool, path.0.clone()).await {
        Ok(post) => Ok(pages::PostPage { post }.render()),
        Err(_) => Ok(pages::NotFoundPage {}.render()),
    }
}

pub async fn edit(path: Path<(String,)>, state: web::Data<State>) -> Result<HttpResponse, Error> {
    let pool = &mut *state.pool.borrow_mut();

    match load_post(pool, path.0.clone()).await {
        Ok(post) => Ok(pages::PostFormPage { post: Some(post) }.render()),
        Err(_) => Ok(pages::NotFoundPage {}.render()),
    }
}

async fn load_post(pool: &mut PgPool, key: String) -> Result<Post, String> {
    sqlx::query_as!(
        Post,
        "select key, title, content, published, created_at, updated_at
            from post where key = $1",
        key
    )
    .fetch_one(pool)
    .await
    .map_err(|e| e.to_string())
}

pub async fn create(
    form: web::Form<PostParams>,
    state: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let pool = &mut *state.pool.borrow_mut();
    let now = Utc::now();

    let result = sqlx::query!(
        "insert into post (title, content, published, created_at, updated_at)
            values ($1, $2, $3, $4, $5) returning key",
        form.title,
        form.content,
        false,
        now,
        now
    )
    .fetch_one(pool)
    .await;

    match result {
        Ok(row) => {
            let slug = slug::slugify(&form.title);
            Ok(redirect(&format!("/posts/{}/read/{}", row.key, slug)))
        }
        Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
    }
}

pub async fn update(
    path: Path<(String,)>,
    form: web::Form<PostParams>,
    state: web::Data<State>,
) -> Result<HttpResponse, Error> {
    let pool = &mut *state.pool.borrow_mut();

    let result = sqlx::query!(
        "update post set title = $1, content = $2, updated_at = now() where key = $3",
        form.title,
        form.content,
        path.0
    )
    .execute(pool)
    .await;

    Ok(match result {
        Ok(_) => redirect_to_post(&path.0, &form.title),
        Err(e) => HttpResponse::BadRequest().body(e.to_string()),
    })
}

fn redirect_to_post(key: &str, title: &str) -> HttpResponse {
    let slug = slug::slugify(title);
    redirect(&format!("/posts/{}/read/{}", key, slug))
}

fn redirect(path: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, path)
        .finish()
        .into_body()
}
