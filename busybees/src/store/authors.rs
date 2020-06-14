//! Models and utilities for managing authors.
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use super::StoreResult;

/// Author information with primary key and password hash.
#[derive(Clone)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
}

/// Author information with primary key.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthorWithoutPassword {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl From<Author> for AuthorWithoutPassword {
    fn from(author: Author) -> AuthorWithoutPassword {
        let Author { id, name, email, ..  } = author;

        AuthorWithoutPassword { id, name, email }
    }
}

/// Attempts to find an `Author` by the given email.
pub async fn find(pool: &PgPool, email: String) -> StoreResult<Author> {
    sqlx::query_as!(
        Author,
        "
        select id, email, name, password_hash
        from author where email = $1
        ",
        email,
    ).fetch_one(pool).await
}
