use sqlx::PgPool;

use super::StoreResult;

pub struct Image {
    pub filename: String,
    pub thumbnail_filename: Option<String>,
    pub width: i16,
    pub height: i16,
    pub kb: Option<i32>,
}

pub async fn create(pool: &PgPool, post_key: &str, props: Image) -> StoreResult<()> {
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
    )
    .fetch_one(pool)
    .await?;

    sqlx::query!(
        "
        insert into post_image (post_id, image_id)
            values ((select id from post where key = $1), $2)
        ",
        post_key.to_owned(),
        image_id.id
    )
    .execute(pool)
    .await?;

    Ok(())
}
