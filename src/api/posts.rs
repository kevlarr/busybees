use crate::{models::Post, ActixResult, State};

use actix_web::{web::{Data, Json, Path}, Error, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostUpdate {
    published: bool,
}

pub async fn update(
    path: Path<(String,)>,
    params: Json<PostUpdate>,
    state: Data<State>,
) -> ActixResult {
    let pool = &mut *state.pool.borrow_mut();

    Ok(match Post::update_status(pool, path.0.clone(), params.published).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::BadRequest().body(e),
    })
}
