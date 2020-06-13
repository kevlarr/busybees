use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::pool::PoolConnection;
use sqlx::{PgConnection, PgPool, Transaction};

use super::StoreResult;

pub trait TitleSlug {
    fn title_slug(&self) -> String {
        slug::slugify(&self.title())
    }

    fn title(&self) -> &str;
}

pub struct Post {
    pub author: Option<String>,
    pub key: String,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub first_image: Option<String>,
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

pub struct PostPreview {
    pub author: Option<String>,
    pub key: String,
    pub title: String,
    pub first_image: Option<String>,
    pub created_at: DateTime<Utc>,
}

pub struct AdminPostPreview {
    pub key: String,
    pub title: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TitleSlug for PostParams {
    fn title(&self) -> &str {
        &self.title
    }
}

impl TitleSlug for PostPreview {
    fn title(&self) -> &str {
        &self.title
    }
}

impl TitleSlug for AdminPostPreview {
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

#[deprecated(note = "Use a view instead of hardcoded query here")]
pub async fn public_previews(pool: &PgPool) -> StoreResult<Vec<PostPreview>> {
    sqlx::query_as!(
        PostPreview,
        "
        select
            author.name as author,
            key,
            title,
            created_at,
            first_image(content)
        from post
        left join author on author.id = post.author_id
        where published
        order by created_at desc
        limit 4
        ",
    ).fetch_all(pool).await
}

pub async fn admin_list(pool: &PgPool) -> StoreResult<Vec<AdminPostPreview>> {
    sqlx::query_as!(
        AdminPostPreview,
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

#[deprecated(note = "Use a view instead of hardcoded query here")]
pub async fn find(pool: &PgPool, key: String) -> StoreResult<Post> {
    sqlx::query_as!(
        Post,
        "
        select
                author.name as author,
                post.key,
                post.title,
                post.content,
                post.published,
                post.created_at,
                post.updated_at,
                first_image(post.content)

        from post
        left join author on author.id = post.author_id
        where key = $1
        ",
        key,
    ).fetch_one(pool).await
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
