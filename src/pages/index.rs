use super::{layout::LayoutPage, Renderable};
use crate::models::PostPreview;
use horrorshow::{html, RenderOnce, Template, TemplateBuffer};

pub struct IndexPage {
    pub posts: Vec<PostPreview>,
}

impl RenderOnce for IndexPage {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let mut posts = self.posts.iter();
        let first = posts.next();

        match first {
            Some(post) => tmpl << html! {
                section (id = "Content") {
                    section (id = "PrimaryPost") {
                        article (class = "primary post") {
                            h1 : &post.title;
                        }
                    }

                    section (id = "SecondaryPosts") {
                        @ for preview in posts {
                            article (class = "secondary post") {
                                a (
                                    href = format!("/posts/{}/read/{}", preview.key, slug::slugify(&preview.title)),
                                    class = "post-link"
                                ) {
                                    h2 : &preview.title;
                                }
                                p : preview.created_at.to_string();
                            }
                        }
                    }
                }
            },
            None => tmpl << html! {
                section (id = "NoContent"): "No posts to display";
            },
        }
    }
}

impl Into<String> for IndexPage {
    fn into(self) -> String {
        LayoutPage {
            title: "New posts".into(),
            main_id: "Index".into(),
            content: self,
        }
        .into_string()
        .unwrap_or_else(|_| "Error generating index".into())
    }
}

impl Renderable for IndexPage {}
