use super::layout::Layout;
use chrono::{DateTime, Utc};
use horrorshow::{html, Raw, RenderOnce, Template, TemplateBuffer};

pub struct Post {
    //id: i32,
    pub title: String,
    pub body: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RenderOnce for Post {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Post {
            title,
            body,
            created_at,
            updated_at,
            ..
        } = self;

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
        Layout {
            title: self.title.clone(),
            main_id: "Post".into(),
            content: self,
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating article page".into())
    }
}

pub struct NewPost;

impl RenderOnce for NewPost {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            div(id = "editor");

            // WYSIWYG editor
            link(rel = "stylesheet", type = "text/css", href = "https://cdn.jsdelivr.net/npm/summernote@0.8.16/dist/summernote-lite.min.css");

            script(src = "https://code.jquery.com/jquery-3.4.1.min.js");
            script(src = "https://cdn.jsdelivr.net/npm/summernote@0.8.16/dist/summernote-lite.min.js");
            script(src = "/public/assets/editor.js");
        };
    }
}

impl Into<String> for NewPost {
    fn into(self) -> String {
        Layout {
            title: "Say something!".into(),
            main_id: "NewPost".into(),
            content: self,
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating new post page".into())
    }
}
