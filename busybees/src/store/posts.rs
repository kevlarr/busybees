use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::pool::PoolConnection;
use sqlx::{PgConnection, PgPool, Transaction};
use super::{
    images::{self, PostImage},
    StoreResult,
};

pub trait TitleSlug {
    fn title_slug(&self) -> String {
        slug::slugify(&self.title())
    }

    fn title(&self) -> &str;
}

pub struct PostDetail {
    pub author: Option<String>,
    pub key: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub published: bool,
    pub preview_image_filename: Option<String>,
    pub preview_image_alt_text: Option<String>,
}

pub struct PostMeta {
    pub author: Option<String>,
    pub key: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub preview_image_filename: Option<String>,
    pub preview_image_alt_text: Option<String>,
}

pub struct AdminPostMeta {
    pub key: String,
    pub title: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Deserialize)]
pub struct PostParams {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePostParams {
    pub post: PostParams,
    pub linked_uploads: Vec<String>,
    pub preview_image_id: Option<i32>,
}

impl TitleSlug for PostParams {
    fn title(&self) -> &str {
        &self.title
    }
}

impl TitleSlug for PostMeta {
    fn title(&self) -> &str {
        &self.title
    }
}

impl TitleSlug for AdminPostMeta {
    fn title(&self) -> &str {
        &self.title
    }
}

pub async fn create(pool: &PgPool, params: PostParams) -> StoreResult<String> {
    sqlx::query!(
        "
        insert into post (title, content, published, created_at, updated_at)
            values ($1, $2, false, now(), now())
            returning key
        ",
        params.title,
        params.content,
    ).fetch_one(pool).await.map(|row| row.key)
}

pub async fn recent_published(pool: &PgPool) -> StoreResult<Vec<PostMeta>> {
    sqlx::query_as!(
        PostMeta,
        "
        select
            author,
            key,
            title,
            created_at,
            preview_image_filename,
            preview_image_alt_text
        from post_published_by_date_vw
        limit 4
        ",
    ).fetch_all(pool).await
}

pub async fn admin_list(pool: &PgPool) -> StoreResult<Vec<AdminPostMeta>> {
    sqlx::query_as!(
        AdminPostMeta,
        "
        select
            key,
            title,
            published,
            created_at,
            updated_at
        from post
        order by created_at desc
        ",
    ).fetch_all(pool).await
}

pub async fn get(pool: &PgPool, key: String) -> StoreResult<PostDetail> {
    sqlx::query_as!(
        PostDetail,
        "
        select
            author,
            key,
            title,
            content,
            created_at,
            published,
            preview_image_filename,
            preview_image_alt_text
        from post_detail_vw
        where key = $1
        ",
        key,
    ).fetch_one(pool).await
}

pub async fn get_with_images(pool: &PgPool, key: String) -> StoreResult<(PostDetail, Vec<PostImage>)> {
    let post = get(pool, key).await?;
    let post_images = images::for_post(pool, &post.key).await?;

    Ok((post, post_images))
}

pub async fn update_post(
    tx: &mut Transaction<PoolConnection<PgConnection>>,
    key: String,
    params: UpdatePostParams,
) -> StoreResult<()> {
    let UpdatePostParams { post, linked_uploads, preview_image_id } = params;
    let PostParams { title, content } = post;

    let post = sqlx::query!(
        "
        update post
        set
            title = $2,
            content = $3,
            updated_at = now()
        where key = $1
        returning id
        ",
        key,
        title,
        content
    ).fetch_one(&mut *tx).await?;

    sqlx::query!(
        "
        delete from post_image
        using image
        where
            image.id = image_id and
            post_id = $1 and
            image.filename <> all($2)
        ",
        post.id,
        &linked_uploads
    ).execute(&mut *tx).await?;

    // This is less than ideal, but until support for dynamic VALUES list drops,
    // I would rather take the performance hit of multiple queries in favor
    // of having compile-time guarantees.
    //
    // See: https://github.com/launchbadge/sqlx/issues/291
    for filename in linked_uploads {
        sqlx::query!(
            "
            insert into post_image (post_id, image_id)
                values ($1, (select id from image where filename = $2))

                -- Conflict could arise not only from trying to save a linked upload
                -- that already exists, but also from a post that links to the same
                -- file twice. The latter probably isn't common, but it shouldn't lead
                -- to a duplicate record.
                on conflict do nothing
            ",
            post.id,
            filename,
        ).execute(&mut *tx).await?;
    }

    if let Some(image_id) = preview_image_id {
        // This two-updates/one-statement approach actually helps to minimize
        // *both* updating rows needlessly *and* number of statements issued.
        // Eg. if the same post-image is selected, 0 updates will be made.
        sqlx::query!(
            "
            -- Clears existing `is_preview` if on different post-image
            with clear_different_preview as (
                update post_image
                set is_preview = false
                where post_id = $1 and image_id != $2 and is_preview
                returning true
            )

            -- Only updates relevant post-image to `is_preview` if unset
            update post_image
            set is_preview = true
            from clear_different_preview
            where post_id = $1 and image_id = $2;
            ",
            post.id,
            image_id,
        ).execute(&mut *tx).await?;
    }

    Ok(())
}

pub async fn update_status(pool: &PgPool, key: String, published: bool) -> StoreResult<()> {
    sqlx::query!(
        "
        update post set published = $2, updated_at = now() where key = $1
        ",
        key,
        published,
    ).execute(pool).await?;

    Ok(())
}

pub async fn delete(pool: &PgPool, key: &str) -> StoreResult<()> {
    sqlx::query!(
        "
        delete from post where key = $1
        ",
        key.to_string(),
    ).execute(pool).await?;

    Ok(())
}
