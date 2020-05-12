use crate::{
    models::{PostPreview, TitleSlug},
    pages::Page,
    ActixResult,
    State,
};

use actix_web::{web::Data, Either, HttpResponse};
use horrorshow::{html, RenderOnce, TemplateBuffer};


pub async fn get(page: Page, state: Data<State>) -> Either<Page, ActixResult> {
    match PostPreview::load_latest(&state.pool).await {
        Ok(posts) => Either::A(
            page.id("Home")
                .title("Latest Posts")
                .content(Home { posts })
        ),
        Err(e) => Either::B(
            // TODO This should be an actual page
            Ok(HttpResponse::BadRequest().body(e))
        ),
    }
}

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
                                img (src = match &preview.first_image {
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
                                    img (src = match &preview.first_image {
                                        Some(s) => s.to_string(),
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
