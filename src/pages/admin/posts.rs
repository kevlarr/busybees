use crate::{
    models::{AdminPostPreview, Post, TitleSlug},
    pages::Page,
    State,
};
use actix_web::{web::{Data, Json, Path}, Error, HttpResponse};
use horrorshow::{html, RenderOnce, TemplateBuffer};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PostUpdate {
    published: bool,
}

pub struct Posts {
    posts: Result<Vec<AdminPostPreview>, String>,
}

impl Posts {
    pub async fn get(page: Page, state: Data<State>) -> Page {
        let pool = &mut *state.pool.borrow_mut();

        page.id("AdminPosts")
            .title("Manage Posts")
            .content(Self {
                posts: AdminPostPreview::load_all(pool).await
            })
    }

    pub async fn update_published(
        page: Page,
        path: Path<(String,)>,
        params: Json<PostUpdate>,
        state: Data<State>,
    ) -> Page {
        let pool = &mut *state.pool.borrow_mut();

        Ok(match Post::update_status(pool, path.0.clone(), params.published).await {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(e) => HttpResponse::BadRequest().body(e),
        })
    }
}

impl RenderOnce for Posts {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Posts { posts } = self;

        match posts {
            Ok(posts) => tmpl << html! {
                admin-posts {
                    @ for post in posts {
                        : PostItem { post };
                    }
                }
            },
            Err(e) => tmpl << html! {
                p : e;
            },
        }
    }
}


pub struct PostItem {
    post: AdminPostPreview,
}

impl RenderOnce for PostItem {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Self { post } = self;

        tmpl << html! {
            admin-post-item {
                form (method = "post", action = "/admin/posts/{}/update") {
                    input (type = "checkbox", checked? = post.published);
                    post-status (type = if post.published { "published" } else { "unlisted" });
                }
                a (class = "admin-post-title", href = format!("/posts/{}/read/{}", &post.key, post.title_slug())) {
                    h2 : &post.title;
                }
            }
        }
    }
}
