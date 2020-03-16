use chrono::{DateTime, Utc};
use serde::Deserialize;


#[derive(Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
}


pub struct Post {
    pub key: String,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
