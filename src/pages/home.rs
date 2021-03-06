use crate::upload_path;
use crate::store::posts::{PublishedPostMeta, PostLink};
use horrorshow::{html, RenderOnce, TemplateBuffer};

pub struct Home {
    pub posts: Vec<PublishedPostMeta>,
}

impl RenderOnce for Home {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let mut posts = self.posts.iter();
        let first = posts.next();

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
                        @ for preview in posts {
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
