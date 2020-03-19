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
            Some(preview) => tmpl << html! {
                section (id = "Content") {
                    section (id = "PrimaryPost") {
                        article (class = "primary post") {
                            img (src = match &preview.first_src {
                                Some(s) => s.to_string(),
                                None => format!("https://picsum.photos/seed/{}/600/300", &preview.key),
                            });
                            a (
                                href = format!("/posts/{}/read/{}", preview.key, slug::slugify(&preview.title)),
                                class = "post-link"
                            ) {
                                h1 : &preview.title;
                            }
                        }
                    }

                    section (id = "SecondaryPosts") {
                        @ for preview in posts {
                            article (class = "secondary post") {
                                //img (src = &preview.first_src);
                                //img (src = if let Some(src) = &preview.first_src { src } else { "https://picsum.photos/800/400" });
                                img (src = match &preview.first_src {
                                    Some(s) => s.to_string(),
                                    None => format!("https://picsum.photos/seed/{}/300/150", &preview.key),
                                });

                                a (
                                    href = format!("/posts/{}/read/{}", preview.key, slug::slugify(&preview.title)),
                                    class = "post-link"
                                ) {
                                    h2 : &preview.title;
                                }
                                //p : preview.created_at.to_string();
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
