use busybees::{
    imaging,
    store::posts::{PostPreview, TitleSlug},
};
use horrorshow::{html, RenderOnce, TemplateBuffer};
use std::path::Path;

#[deprecated(note = "Fix behavior of `thumbnail`")]
pub struct Home {
    pub posts: Vec<PostPreview>,
}

impl RenderOnce for Home {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let mut posts = self.posts.iter();
        let first = posts.next();

        match first {
            Some(preview) => {
                tmpl << html! {
                    section (id = "PrimaryPost") {
                        a (
                            href = format!("/posts/{}/read/{}", preview.key, preview.title_slug()),
                            class = "primary post-link"
                        ) {
                            preview (type = "primary") {
                                img (src = match &preview.thumbnail {
                                    // DEPRECATED
                                    // For now, assume "thumbnail" refers actually to the first
                                    // full image, so just return it directly
                                    Some(s) => s.to_string(),
                                    None => format!("https://picsum.photos/seed/{}/600/300", &preview.key),
                                });
                                footer {
                                    h1 : &preview.title;
                                    @ if let Some(name) = &preview.author {
                                        : "by ";
                                        post-author : name;
                                        : " on ";
                                    }
                                    post-published : &preview.created_at.format("%a %b %e, %Y").to_string();
                                }
                            }
                        }
                    }

                    section (id = "SecondaryPosts") {
                        @ for preview in posts {
                            a (
                                href = format!("/posts/{}/read/{}", preview.key, preview.title_slug()),
                                class = "secondary post-link"
                            ) {
                                preview (type = "secondary") {
                                    img (src = match &preview.thumbnail {
                                        // DEPRECATED
                                        // Assumes that `thumbnail` is actually a full image, so
                                        // use the helper to generate the thumbnail path
                                        Some(s) => {
                                            let p = Path::new(&s);
                                            imaging::thumbnail_path(&p)
                                                .map(|pathbuf| pathbuf.into_boxed_path())
                                                .map(|path| imaging::path_filename(&path)) // result<result<..>>
                                                .map(|result| result.unwrap_or_else(|_|
                                                    format!("https://picsum.photos/seed/{}/300/150", &preview.key)
                                                ))
                                                .unwrap_or_else(|_|
                                                    format!("https://picsum.photos/seed/{}/300/150", &preview.key)
                                                )
                                        },
                                        None => format!("https://picsum.photos/seed/{}/300/150", &preview.key),
                                    });

                                    footer {
                                        h2 : &preview.title;
                                        post-meta {
                                            @ if let Some(name) = &preview.author {
                                                : "by ";
                                                post-author : name;
                                                : " on ";
                                            }
                                            post-published : &preview.created_at.format("%a %b %e, %Y").to_string();
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
