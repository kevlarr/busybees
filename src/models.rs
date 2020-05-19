use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;


pub struct Image {
    pub src: String,
    pub thumbnail_src: Option<String>,
    pub width: Option<i16>,
    pub height: Option<i16>,
    pub kb: Option<i32>,
}

impl Image {
    pub async fn create(pool: &PgPool, props: Image) -> Result<(), String> {
        sqlx::query!(r#"
            insert into image (src, thumbnail_src, width, height, kb)
                values ($1, $2, $3, $4, $5)
        "#, props.src, props.thumbnail_src, props.width, props.height, props.kb)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}


#[derive(Deserialize)]
pub struct PostProps {
    pub title: String,
    pub content: String,
}

pub struct PostPreview {
    pub author: Option<String>,
    pub key: String,
    pub title: String,
    pub first_image: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl PostPreview {
    pub async fn load_latest(pool: &PgPool) -> Result<Vec<Self>, String> {
        sqlx::query_as!(Self, r#"
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
        "#).fetch_all(pool).await.map_err(|e| e.to_string())
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
    pub first_image: Option<String>
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
        sqlx::query_as!(Self, "
            select
                author.name as author,
                post.key,
                post.title,
                post.content,
                post.published,
                post.created_at,
                post.updated_at,
                first_image(post.content)

            from post left join author on author.id = post.author_id
            where key = $1
        ", key).fetch_one(pool).await.map_err(|e| e.to_string())
    }

    pub async fn delete(pool: &PgPool, key: &str) -> Result<(), String> {
        sqlx::query!("delete from post where key = $1", key.to_string())
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }
}

#[derive(Clone)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

impl Author {
    pub async fn load(pool: &PgPool, email: String) -> Result<Self, String> {
        sqlx::query_as!(
            Self,
            "select id, email, name, password_hash from author where email = $1",
            email,
        ).fetch_one(pool).await.map_err(|e| e.to_string())
    }
}

impl Into<AuthorWithoutPassword> for Author {
    fn into(self) -> AuthorWithoutPassword {
        let Author { id, name, email, .. } = self;

        AuthorWithoutPassword { id, name, email }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorWithoutPassword {
    pub id: i32,
    pub name: String,
    pub email: String,
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
