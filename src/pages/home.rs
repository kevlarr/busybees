use crate::{models::PostPreview, pages::Page, ActixResult, State};

use actix_web::{web::Data, Either, HttpResponse};
use horrorshow::{html, RenderOnce, TemplateBuffer};


pub async fn get(page: Page, state: Data<State>) -> Either<Page, ActixResult> {
    let pool = &mut *state.pool.borrow_mut();

    let result = sqlx::query_as!(PostPreview, r#"
        select
            key,
            title,
            created_at,
            substring(content, 'src="([a-zA-Z0-9\.\-_~:\/%\?#=]+)"') as first_src
        from post
        where published
        order by created_at desc
        limit 4
    "#).fetch_all(pool).await;

    match result {
        Ok(posts) => Either::A(
            page.id("Home")
                .title("Latest Posts")
                .content(Home { posts })
        ),
        Err(e) => Either::B(
            // TODO This should be an actual page
            Ok(HttpResponse::BadRequest().body(e.to_string()))
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
                            href = format!("/posts/{}/read/{}", preview.key, slug::slugify(&preview.title)),
                            class = "primary post-link"
                        ) {
                            preview (type = "primary") {
                                img (src = match &preview.first_src {
                                    Some(s) => s.to_string(),
                                    None => format!("https://picsum.photos/seed/{}/600/300", &preview.key),
                                });
                                footer {
                                    h1 : &preview.title;
                                    time : &preview.created_at.format("%a %b %e, %Y @ %l:%M %P %Z").to_string();
                                }
                            }
                        }
                    }

                    section (id = "SecondaryPosts") {
                        @ for preview in posts {
                            a (
                                href = format!("/posts/{}/read/{}", preview.key, slug::slugify(&preview.title)),
                                class = "secondary post-link"
                            ) {
                                preview (type = "secondary") {
                                    img (src = match &preview.first_src {
                                        Some(s) => s.to_string(),
                                        None => format!("https://picsum.photos/seed/{}/300/150", &preview.key),
                                    });
                                    footer {
                                        h2 : &preview.title;
                                        time : &preview.created_at.format("%a %b %e, %Y @ %l:%M %P %Z").to_string();
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
