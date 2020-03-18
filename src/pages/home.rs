use super::layout::Layout;
use chrono::{DateTime, Utc};
use horrorshow::{html, Raw, RenderOnce, Template, TemplateBuffer};

pub struct Home {
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RenderOnce for Home {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Post { title, content, published, created_at, updated_at } = self;

        tmpl << html! {
            h1 : title;
            p : Raw(content);
            p : format!("Published: {}", published);
            p : format!("Created: {}", created_at);
            p : format!("Updated: {}", updated_at);
        };
    }
}

impl Into<String> for Post {
    fn into(self) -> String {
        Layout {
            title: self.title.clone(),
            main_id: "Post".into(),
            content: self,
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating article page".into())
    }
}
