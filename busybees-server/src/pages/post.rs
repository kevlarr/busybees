use horrorshow::{html, Raw, RenderOnce, TemplateBuffer};
use busybees::store::posts::PostDetail;
use crate::asset_path;

pub struct PostView {
    pub post: PostDetail,
    pub auth: bool,
}

impl RenderOnce for PostView {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let auth = self.auth;
        let PostDetail {
            author,
            key,
            title,
            content,
            published,
            created_at,
            ..
        } = self.post;

        tmpl << html! {
            h1 : title;
            post-meta {
                @ if auth {
                    post-status (
                        type = if published { "published" } else { "unlisted" },
                        data-post-key = &key
                    );
                }
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
