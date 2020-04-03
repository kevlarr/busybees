use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

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

pub struct Post {
    pub key: String,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password_hash: String,
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
