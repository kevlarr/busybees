mod models;
mod queries;

pub use models::*;
pub use queries::*;

/// Generates a URL-friendly slug from the given field.
pub trait PostSlug {
    fn to_slug(&self) -> String {
        slug::slugify(&self.slug_field())
    }

    fn slug_field(&self) -> &str;
}

impl PostSlug for AdminPostMeta {
    fn slug_field(&self) -> &str { &self.title }
}

impl PostSlug for PostDetail {
    fn slug_field(&self) -> &str { &self.title }
}

impl PostSlug for PublishedPostMeta {
    fn slug_field(&self) -> &str { &self.title }
}

impl PostSlug for PostParams {
    fn slug_field(&self) -> &str { &self.title }
}

/// Generates an anchor HREF from key field and slug.
#[deprecated(note = "this is page-specific and doesn't belong in store")]
pub trait PostLink : PostSlug {
    fn href(&self) -> String {
        format!("/posts/{}/read/{}", &self.key_field(), self.to_slug())
    }

    fn key_field(&self) -> &str;
}

impl PostLink for AdminPostMeta {
    fn key_field(&self) -> &str { &self.key }
}

impl PostLink for PostDetail {
    fn key_field(&self) -> &str { &self.key }
}

impl PostLink for PublishedPostMeta {
    fn key_field(&self) -> &str { &self.key }
}
