use serde::{Serialize, Deserialize};
use sqlx::PgPool;

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
