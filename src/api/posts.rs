use crate::{ActixResult, State};
use crate::models::{Post, PostProps};

use actix_web::{web::{Data, Json, Path}, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdatePublishedParams {
    published: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePostParams {
    pub post: PostProps,
    pub linked_uploads: Vec<String>,
}

pub async fn update(
    path: Path<(String,)>,
    props: Json<UpdatePostParams>,
    state: Data<State>,
) -> ActixResult {
    let key = &path.0;
    let mut tx = state.pool.begin().await
        .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))?;

    let UpdatePostParams { post, linked_uploads } = props.into_inner();
    let PostProps { title, content } = post;

    sqlx::query!(
        "delete from post_image where post_id = (
            select id from post where key = $1
        )", key.clone()
    )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            eprintln!("{}", msg);
            HttpResponse::BadRequest().body(msg)
        })?;

    sqlx::query!(r#"
        update post set title = $2, content = $3, updated_at = now() where key = $1
    "#, key.clone(), title, content)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            let msg = e.to_string();
            eprintln!("{}", msg);
            HttpResponse::BadRequest().body(msg)
        })?;

    // This is less than ideal, but until support for dynamic VALUES list drops...
    // (see https://github.com/launchbadge/sqlx/issues/291)
    // ... I would rather take the performance hit of multiple queries in favor
    // of having compile-time guarantees.
    for filename in linked_uploads {
        sqlx::query!("
            insert into post_image (post_id, image_id)
                values (
                    (select id from post where key = $1),
                    (select id from image where filename = $2)
                )

                -- It is dumb to try linking to an image embedded in the same post,
                -- but it's not worth an error at all. Just don't need an extra record.
                on conflict do nothing",
            key.clone(),
            filename,
        )
            .execute(&mut *tx)
            .await
            .map_err(|e| {
                let msg = e.to_string();
                eprintln!("{}", msg);
                HttpResponse::BadRequest().body(msg)
            })?;
    }

    tx.commit().await.map_err(|e| {
        let msg = e.to_string();
        eprintln!("{}", msg);
        HttpResponse::InternalServerError().body(msg)
    })?;

    Ok(HttpResponse::NoContent().finish())
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
            HttpResponse::BadRequest().body(e.to_string())
        },
    })
}
