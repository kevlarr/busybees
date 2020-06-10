use horrorshow::{html, Raw, RenderOnce, TemplateBuffer};

use crate::{
    store::posts::Post,
    asset_path,
};

pub struct PostView {
    pub post: Post,
    pub auth: bool,
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
                script(src = asset_path("admin.js"));
            }
        };
    }
}
