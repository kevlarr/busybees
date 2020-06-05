use actix_web::{web::{Data, Json, Path}, HttpResponse};
use serde::Deserialize;

use crate::{ApiResult, State};
use crate::store::posts::{self, UpdatePostParams};

#[derive(Deserialize)]
pub struct UpdatePublishedParams {
    published: bool,
}

#[deprecated(note = "Use ApiResult and ApiError instead of `map_err`")]
pub async fn update(
    path: Path<(String,)>,
    props: Json<UpdatePostParams>,
    state: Data<State>,
) -> ApiResult<HttpResponse> {
    let mut tx = state.pool.begin().await?;
    let key = &path.0;

    posts::update_post(&mut tx, key.clone(), props.into_inner()).await?;
    tx.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn update_published(
    path: Path<(String,)>,
    props: Json<UpdatePublishedParams>,
    state: Data<State>,
) -> ApiResult<HttpResponse> {
    posts::update_status(&state.pool, path.0.clone(), props.published).await?;
    Ok(HttpResponse::NoContent().finish())
}
