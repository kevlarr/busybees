use super::layout::Layout;
use chrono::{DateTime, Utc};
use horrorshow::{html, Raw, RenderOnce, Template, TemplateBuffer};

pub struct Post {
    pub content: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RenderOnce for Post {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Post { content, published, created_at, updated_at } = self;

        tmpl << html! {
            p : format!("Published: {}", published);
            p : format!("Created: {}", created_at);
            p : format!("Updated: {}", updated_at);
            p : Raw(content);
        };
    }
}

impl Into<String> for Post {
    fn into(self) -> String {
        Layout {
            title: self.content.chars().take(20).collect(),
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
            form(id = "EditorForm", method = "post", action = "/posts/new") {
                textarea(id = "SummernoteEditor", name = "content");
                input(id = "PostTitle", name = "title",    /* hidden = "true", */ readonly = "true");
                input(id = "PostAlpha", name = "alpha_id", /* hidden = "true", */ readonly = "true");

                div(id = "PostControls") {
                    button(id = "CancelEditor") : "Cancel";
                    button(id = "SubmitEditor", type = "submit", class = "primary", disabled = "true") : "Submit";
                }
            }

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
