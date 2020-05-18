use crate::{ActixResult, State};
use crate::models::{Post, PostProps};

use actix_web::{web::{Data, Json, Path}, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdatePublishedParams {
    published: bool,
}

pub async fn update(
    path: Path<(String,)>,
    props: Json<PostProps>,
    state: Data<State>,
) -> ActixResult {
    Ok(match Post::update(&state.pool, path.0.clone(), props.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("{}", e.to_string());
            HttpResponse::BadRequest().finish()
        },
    })
}

pub async fn update_published(
    path: Path<(String,)>,
    props: Json<UpdatePublishedParams>,
    state: Data<State>,
) -> ActixResult {
    Ok(match Post::update_status(&state.pool, path.0.clone(), props.published).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            eprintln!("{}", e.to_string());
            HttpResponse::BadRequest().body(e)
        },
    })
}
