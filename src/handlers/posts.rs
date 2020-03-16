use actix_web::{
    http,
    web::{self, Path},
    Error, HttpResponse,
};
use chrono::Utc;
use crate::{
    models,
    pages::{self, Renderable},
    State,
};


fn redirect(path: &str) -> HttpResponse {
    HttpResponse::Found()
        .header(http::header::LOCATION, path)
        .finish()
        .into_body()
}


//pub async fn index(state: web::Data<State>) -> Result<HttpResponse, Error> {
    //let pool = &mut *state.pool.borrow_mut();

    //let result = sqlx::query!("
        //select key, title, substring(content from 0 for 256), created_at
        //from post
    //")
        //.fetch_one(pool)
        //.await;

    //if let Err(e) = result {
        //return Ok(HttpResponse::BadRequest().body(e.to_string()));
    //}
//}

pub async fn show(
    path: Path<(String,)>,
    state: web::Data<State>
) -> Result<HttpResponse, Error>  {
    let pool = &mut *state.pool.borrow_mut();
    let key = match path.0.splitn(2, '-').next() {
        Some(k) => k,
        None => return Ok(HttpResponse::NotFound().finish()),
    };

    let result = sqlx::query_as!(models::Post, "
        select key, title, content, published, created_at, updated_at
        from post where key = $1
    ", key.to_string()).fetch_one(pool).await;

    match result {
        Ok(post) => Ok(pages::PostPage{ post }.render()),
        Err(_) => Ok(pages::NotFoundPage{}.render()),
    }
}


pub async fn create(
    form: web::Form<models::NewPost>,
    state: web::Data<State>
) -> Result<HttpResponse, Error>  {
    let pool = &mut *state.pool.borrow_mut();

    let now = Utc::now();

    let result = sqlx::query!("
        insert into post
        (title, content, published, created_at, updated_at)
            values ($1, $2, $3, $4, $5)
        returning key
    ", form.title, form.content, false, now, now)
        .fetch_one(pool)
        .await;

    match result {
        Ok(row) => {
            let slug = slug::slugify(&form.title);

            Ok(redirect(&format!("/posts/{}-{}", row.key, slug)))
        },
        Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
    }
}
