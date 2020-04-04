use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}

pub struct PostPreview {
    pub key: String,
    pub title: String,
    pub first_src: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl PostPreview {
    pub async fn load_latest(pool: &mut PgPool) -> Result<Vec<Self>, String> {
        sqlx::query_as!(Self, r#"
            select
                key,
                title,
                created_at,
                substring(content, 'src="([a-zA-Z0-9\.\-_~:\/%\?#=]+)"') as first_src
            from post
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
    pub async fn load_all(pool: &mut PgPool) -> Result<Vec<Self>, String> {
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
    pub key: String,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Post {
    pub async fn create(pool: &mut PgPool, params: NewPost) -> Result<String, String> {
        let now = Utc::now();

        sqlx::query!(r#"
            insert into post (title, content, published, created_at, updated_at)
                values ($1, $2, $3, $4, $5)
                returning key
        "#, params.title, params.content, false, now, now)
            .fetch_one(pool)
            .await
            .map(|row| row.key)
            .map_err(|e| e.to_string())
    }

    pub async fn update(pool: &mut PgPool, key: String, params: NewPost) -> Result<(), String> {
        sqlx::query!(r#"
            update post set title = $2, content = $3, updated_at = now() where key = $1
        "#, key, params.title, params.content)
            .execute(pool)
            .await
            .map(|_| ())
            .map_err(|e| e.to_string())
    }

    pub async fn load(pool: &mut PgPool, key: String) -> Result<Self, String> {
        sqlx::query_as!(
            Self,
            "select key, title, content, published, created_at, updated_at
                from post where key = $1",
            key
        ).fetch_one(pool).await.map_err(|e| e.to_string())
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
    pub async fn load(pool: &mut PgPool, email: String) -> Result<Self, String> {
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
