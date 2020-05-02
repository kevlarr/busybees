use crate::{
    models::{AdminPostPreview, Post, TitleSlug},
    pages::Page,
    ActixResult,
    State,
    asset_path,
    redirect,
};
use actix_web::{web::{Data, Path}, HttpResponse};
use horrorshow::{html, RenderOnce, TemplateBuffer};


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

    pub async fn delete(path: Path<(String,)>, state: Data<State>) -> ActixResult {
        let pool = &mut *state.pool.borrow_mut();

        match Post::delete(pool, &path.0).await {
            Ok(()) => Ok(redirect("/admin/posts")),
            Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
        }
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
                script(src = asset_path("admin.js"));
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
                post-status (
                    type = if post.published { "published" } else { "unlisted" },
                    data-post-key = &post.key
                );

                a (class = "admin-post-title", href = format!("/posts/{}/read/{}", &post.key, post.title_slug())) {
                    h2 : &post.title;
                }
            }
        }
    }
}
