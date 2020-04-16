use crate::{
    models::Post,
    pages::{notfound, Page},
    State,
};

use actix_web::{
    web::{self, Data, Path},
    Scope,
};
use horrorshow::{html, Raw, RenderOnce, TemplateBuffer};


pub fn resource(path: &str) -> Scope {
    web::scope(path)
        .route("/{key}/read/{slug}", web::get().to(PostView::get))
}


pub struct PostView {
    pub post: Post,
    pub auth: bool,
}

impl PostView {
    pub async fn get(
        page: Page,
        path: Path<(String, String)>,
        state: Data<State>,
    ) -> Page {
        let pool = &mut *state.pool.borrow_mut();
        let auth = page.user.is_some();

        match Post::load(pool, path.0.clone()).await {
            Ok(post) => page
                .id("Post")
                .title(post.title.clone())
                .image(post.first_image.clone())
                .content(Self{ auth, post }),

            Err(_) => notfound::get_sync(page),
        }
    }
}

impl RenderOnce for PostView {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let auth = self.auth;
        let Post {
            author,
            key,
            title,
            content,
            published,
            created_at,
            ..
        } = self.post;

        tmpl << html! {
            @ if auth {
                post-controls {
                    post-status (
                        type = if published { "published" } else { "unlisted" },
                        data-post-key = &key
                    );
                    post-changes {
                        a (class = "icon-link", href = format!("/admin/posts/edit/{}", key)) {
                            i (class = "fa fa-pencil-square-o");
                            : " Edit";
                        }
                        a (class = "icon-link", href = format!("/admin/posts/delete/{}", key)) {
                            i (class = "fa fa-trash-o");
                            : " Delete";
                        }
                    }
                }
            }

            h1 : title;
            post-meta {
                @ if let Some(name) = author {
                    : "by ";
                    post-author : name;
                    : " on ";
                }
                post-published : created_at.format("%a %b %e, %Y").to_string();
            }
            post-content : Raw(content);

            @ if auth {
                script(src = "/public/assets/admin.js");
            }
        };
    }
}
