use crate::models::Post;
use horrorshow::{html, Raw, RenderOnce, Template, TemplateBuffer};
use super::{layout::LayoutPage, Renderable};

pub struct PostPage {
    pub post: Post,
}

impl RenderOnce for PostPage {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Post { title, content, published, created_at, updated_at, .. } = self.post;

        tmpl << html! {
            h1 : title;
            p : Raw(content);
            p : format!("Published: {}", published);
            p : format!("Created: {}", created_at);
            p : format!("Updated: {}", updated_at);
        };
    }
}

impl Into<String> for PostPage {
    fn into(self) -> String {
        LayoutPage {
            title: self.post.title.clone(),
            main_id: "Post".into(),
            content: self,
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating article page".into())
    }
}

impl Renderable for PostPage {}


pub struct NewPostPage;

impl RenderOnce for NewPostPage {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            form(id = "EditorForm", method = "post", action = "/posts/new") {
                input(id = "PostTitle", name = "title", placeholder = "Some clever title here...", autofocus = "true");
                textarea(id = "SummernoteEditor", name = "content");

                div(id = "PostControls") {
                    a (href = "/") { button(type = "button") : "Cancel"; }
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

impl Into<String> for NewPostPage {
    fn into(self) -> String {
        LayoutPage {
            title: "Say something!".into(),
            main_id: "NewPost".into(),
            content: self,
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating new post page".into())
    }
}

impl Renderable for NewPostPage {}
