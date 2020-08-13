use chrono::{DateTime, Utc};
use serde::Deserialize;

pub struct PostDetail {
    pub author: String,
    pub key: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub published: bool,
    pub preview_image_filename: Option<String>,
    pub preview_image_alt_text: Option<String>,
}

pub struct PostMeta {
    pub author: String,
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
    pub author_id: i32,
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
