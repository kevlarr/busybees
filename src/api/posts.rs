use crate::{models::Post, ActixResult, State};

use actix_web::{web::{Data, Json, Path}, HttpResponse};
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
    Ok(match Post::update_status(&state.pool, path.0.clone(), params.published).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => HttpResponse::BadRequest().body(e),
    })
}
