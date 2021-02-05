use horrorshow::{html, Raw, RenderOnce, TemplateBuffer};
use crate::asset_path;
use crate::store::posts::PostDetail;

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
            published_at,
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
                : "by ";
                post-author : author;

                @ if let Some(p) = published_at {
                    : " on ";
                    post-published : p.format("%a %b %e, %Y").to_string();
                }
            }

            post-content : Raw(content);

            @ if auth {
                script(src = asset_path("admin.js"));
            }
        };
    }
}
