use crate::models::PostPreview;
use horrorshow::{html, RenderOnce, Template, TemplateBuffer};
use super::{layout::LayoutPage, Renderable};

pub struct IndexPage {
    pub posts: Vec<PostPreview>,
}

impl RenderOnce for IndexPage {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1 : "Newest articles";

            @ for preview in self.posts {
                a (href = format!("/posts/{}/read/{}", preview.key, slug::slugify(&preview.title))) {
                    h2 : preview.title;
                }
                p : preview.created_at.to_string();
                hr;
            }
        };
    }
}

impl Into<String> for IndexPage {
    fn into(self) -> String {
        LayoutPage {
            title: "New posts".into(),
            main_id: "Post".into(),
            content: self,
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating article page".into())
    }
}

impl Renderable for IndexPage {}
