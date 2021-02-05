use chrono::{DateTime, NaiveDate, Utc};
use serde::Deserialize;

pub struct PostDetail {
    pub author: String,
    pub key: String,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub preview_image_filename: Option<String>,
    pub preview_image_alt_text: Option<String>,
}

pub struct PublishedPostMeta {
    pub author: String,
    pub key: String,
    pub title: String,
    pub published_at: DateTime<Utc>,
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

pub struct NewPostParams {
    pub author_id: i32,
    pub title: String,
    pub content: String,
}

#[derive(Clone, Deserialize)]
pub struct PostParams {
    pub title: String,
    pub content: String,
    pub published_at_date: Option<NaiveDate>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePostParams {
    pub post: PostParams,
    pub linked_uploads: Vec<String>,
    pub preview_image_id: Option<i32>,
}
