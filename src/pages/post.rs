use chrono::{DateTime, Utc};
use horrorshow::{
    html,
    Raw,
    RenderOnce,
    TemplateBuffer,
    Template,
};
use super::layout::Layout;

pub struct Post {
    //id: i32,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RenderOnce for Post {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Post { title, body, created_at, updated_at, .. } = self;

        tmpl << html! {
            h1 : title;
            p : format!("Created: {}", created_at);
            p : format!("Updated: {}", updated_at);
            p : Raw(body);
        };
    }
}

impl Into<String> for Post {
    fn into(self) -> String {
        Layout { title: self.title.clone(), content: self, }
            .into_string()
            .unwrap_or_else(|_| "There was an error generating article page".into())
    }
}
