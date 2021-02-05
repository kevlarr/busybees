//! Basic image modeling and creation.
use serde::Serialize;
use sqlx::PgPool;
use super::StoreResult;

/// Stores filenames for both the image and the associated thumbnail
/// along with basic statistics for the base image.
#[derive(Clone)]
pub struct Image {
    pub filename: String,
    pub thumbnail_filename: Option<String>,
    pub width: i16,
    pub height: i16,
    pub kb: Option<i32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostImage {
    pub image_id: i32,
    pub filename: String,
    pub is_preview: bool,
}

/// Attempts to create an `Image` with the provided properties
/// and then associates it to the `Post` with the given key.
pub async fn create(pool: &PgPool, post_key: &str, props: Image) -> StoreResult<i32> {
    let mut tx = pool.begin().await?;

    let image_id = sqlx::query!(
        "
        insert into image (filename, thumbnail_filename, width, height, kb)
            values ($1, $2, $3, $4, $5)
            returning id
        ",
        props.filename,
        props.thumbnail_filename,
        props.width,
        props.height,
        props.kb,
    ).fetch_one(&mut *tx).await?;

    sqlx::query!(
        "
        insert into post_image (post_id, image_id)
            values ((select id from post where key = $1), $2)
        ",
        post_key.to_owned(),
        image_id.id
    ).execute(&mut *tx).await?;

    tx.commit().await?;

    Ok(image_id.id)
}

#[deprecated(note = "use a view")]
pub async fn for_post(pool: &PgPool, post_key: &str) -> StoreResult<Vec<PostImage>> {
    sqlx::query_as!(
        PostImage,
        "
        select
            image_id,
            coalesce(thumbnail_filename, filename) as filename,
            is_preview
        from post_image
        join image on image.id = post_image.image_id
        join post on post.id = post_image.post_id
        where post.key = $1
        order by image_id
        ",
        post_key
    ).fetch_all(pool).await
}
