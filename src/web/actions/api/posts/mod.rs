use actix_web::{
    web::{Data, Json, Path},
    HttpResponse,
};
use crate::{ApiResult, State};
use crate::store::{self, posts::UpdatePostParams};
use serde::Deserialize;

pub mod images;

#[derive(Deserialize)]
pub struct UpdatePublishedParams {
    published: bool,
}

pub async fn update(
    Path((key,)): Path<(String,)>,
    props: Json<UpdatePostParams>,
    state: Data<State>,
) -> ApiResult<HttpResponse> {
    let mut tx = state.pool.begin().await?;

    store::posts::update_post(&mut tx, key.clone(), props.into_inner()).await?;
    tx.commit().await?;

    Ok(HttpResponse::NoContent().finish())
}

pub async fn update_published(
    Path((key,)): Path<(String,)>,
    props: Json<UpdatePublishedParams>,
    state: Data<State>,
) -> ApiResult<HttpResponse> {
    store::posts::update_status(&state.pool, key.clone(), props.published).await?;
    Ok(HttpResponse::NoContent().finish())
}
