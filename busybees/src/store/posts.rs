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
    let UpdatePostParams {
        post,
        linked_uploads,
    } = params;
    let PostParams { title, content } = post;

    sqlx::query!(
        "
        delete from post_image
        where post_id = (
            select id from post where key = $1
        )
        ",
        key.clone()
    ).execute(&mut *tx).await?;

    sqlx::query!(
        "
        update post set
            title = $2,
            content = $3,
            updated_at = now()
        where key = $1
        ",
        key.clone(),
        title,
        content
    ).execute(&mut *tx).await?;

    // This is less than ideal, but until support for dynamic VALUES list drops...
    // (see https://github.com/launchbadge/sqlx/issues/291)
    // ... I would rather take the performance hit of multiple queries in favor
    // of having compile-time guarantees.
    for filename in linked_uploads {
        sqlx::query!(
            "
            insert into post_image (post_id, image_id)
                values (
                    (select id from post where key = $1),
                    (select id from image where filename = $2)
                )

                -- It is dumb to try linking to an image embedded in the same post,
                -- but it's not worth an error at all. Just don't need an extra record.
                on conflict do nothing
            ",
            key,
            filename,
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
