use crate::upload_path;
use crate::store::posts::{PublishedPostMeta, PostLink};
use horrorshow::{html, RenderOnce, TemplateBuffer};

pub struct Root {
    previews: Vec<PublishedPostMeta>,
}

impl Root {
    pub fn from_action(page: Page, previews: Vec<PublishedPostMeta>) -> Page {
        page.id("Home")
            .title("Latest Posts")
            .content(Self { previews }),
    }
}

impl RenderOnce for Root {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let mut previews = self.previews.iter();
        let first = previews.next();

        match first {
            Some(preview) => {
                tmpl << html! {
                    section (id = "PrimaryPost") {
                        a (href = preview.href(), class = "primary post-link") {
                            preview (type = "primary") {
                                img (src = match &preview.preview_image_filename {
                                    Some(s) => upload_path(s),
                                    None => format!("https://picsum.photos/seed/{}/600/300", &preview.key),
                                });
                                footer {
                                    h1 : &preview.title;
                                    : "by ";
                                    post-author : &preview.author;
                                    : " on ";
                                    post-published : &preview.published_at.format("%a %b %e, %Y").to_string();
                                }
                            }
                        }
                    }

                    section (id = "SecondaryPosts") {
                        @ for preview in previews {
                            a (href = preview.href(), class = "secondary post-link") {
                                preview (type = "secondary") {
                                    img (src = match &preview.preview_image_filename {
                                        Some(s) => upload_path(s),
                                        None => format!("https://picsum.photos/seed/{}/300/150", &preview.key),
                                    });

                                    footer {
                                        h2 : &preview.title;
                                        post-meta {
                                            : "by ";
                                            post-author : &preview.author;
                                            : " on ";
                                            post-published : &preview.published_at.format("%a %b %e, %Y").to_string();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            None => {
                tmpl << html! {
                    section (id = "NoContent"): "No posts to display";
                }
            }
        }
    }
}
