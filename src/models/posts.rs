use chrono::{DateTime, Utc};
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct PostProps {
    pub title: String,
    pub content: String,
}

pub struct PostPreview {
    pub author: Option<String>,
    pub key: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub thumbnail: Option<String>,
    pub alt_text: Option<String>,
}

impl PostPreview {
    pub async fn load_latest(pool: &PgPool) -> Result<Vec<Self>, String> {
        sqlx::query_as!(Self, "select * from post_preview_vw limit 4")
            .fetch_all(pool).await.map_err(|e| e.to_string())
    }
}

pub struct AdminPostPreview {
    pub key: String,
    pub title: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AdminPostPreview {
    pub async fn load_all(pool: &PgPool) -> Result<Vec<Self>, String> {
        sqlx::query_as!(Self, r#"
            select
                key,
                title,
                published,
                created_at,
                updated_at
            from post
            order by created_at desc
        "#).fetch_all(pool).await.map_err(|e| e.to_string())
    }
}

pub struct Post {
    pub author: Option<String>,
    pub key: String,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub thumbnail: Option<String>,
    pub alt_text: Option<String>,
}

impl Post {
    pub async fn create(pool: &PgPool, props: PostProps) -> Result<String, String> {
        let now = Utc::now();

        sqlx::query!(r#"
            insert into post (title, content, published, created_at, updated_at)
                values ($1, $2, $3, $4, $5)
                returning key
        "#, props.title, props.content, false, now, now)
            .fetch_one(pool)
            .await
            .map(|row| row.key)
            .map_err(|e| e.to_string())
    }

    pub async fn update(pool: &PgPool, key: String, props: PostProps) -> Result<(), String> {
        sqlx::query!(r#"
            update post set title = $2, content = $3, updated_at = now() where key = $1
        "#, key, props.title, props.content)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    pub async fn update_status(pool: &PgPool, key: String, published: bool) -> Result<(), String> {
        sqlx::query!(r#"
            update post set published = $2, updated_at = now() where key = $1
        "#, key, published)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    pub async fn load(pool: &PgPool, key: String) -> Result<Self, String> {
        sqlx::query_as!(Self, "select * from post_detail_vw where key = $1", key)
            .fetch_one(pool).await.map_err(|e| e.to_string())
    }

    pub async fn delete(pool: &PgPool, key: &str) -> Result<(), String> {
        sqlx::query!("delete from post where key = $1", key.to_string())
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

pub trait TitleSlug {
    fn title_slug(&self) -> String {
        slug::slugify(&self.title())
    }

    fn title(&self) -> &str;
}

impl TitleSlug for PostProps {
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
