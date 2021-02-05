use sqlx::Error as SqlxError;
use horrorshow::{html, RenderOnce, TemplateBuffer};

use crate::asset_path;
use crate::store::posts::{AdminPostMeta, PostDetail, PostLink};

pub struct Posts {
    pub posts: Result<Vec<AdminPostMeta>, SqlxError>,
}

impl RenderOnce for Posts {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Posts { posts } = self;

        match posts {
            Ok(posts) => {
                tmpl << html! {
                    admin-posts {
                        @ for post in posts {
                            : PostItem { post };
                        }
                    }
                    script(src = asset_path("admin.js"));
                }
            }
            Err(e) => {
                tmpl << html! {
                    p : e.to_string();
                }
            }
        }
    }
}

pub struct PostItem {
    post: AdminPostMeta,
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

                a (class = "admin-post-title", href = post.href()) {
                    h2 : &post.title;
                }
            }
        }
    }
}

pub struct PostForm {
    pub post: PostDetail,
}

impl RenderOnce for PostForm {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let pub_date = &self.post.published_at.map(|dt| dt.format("%F").to_string());

        tmpl << html! {
            form (id = "editorForm", data-post-key = &self.post.key) {
                div (id = "postMeta") {
                    input (id = "postTitle", name = "title", placeholder = "Title", autofocus = "true", value = &self.post.title);
                    input (id = "postPublishedDate", name = "published_at_date", type = "date", value = pub_date);
                }
                fieldset (id = "postImagesSet") {
                    legend : "Cover Image";
                    div (id = "postImages");
                }
                textarea (id = "summernoteEditor", name = "content") : &self.post.content;
            }
            div (id = "form-meta") {
                div (id = "save-status") {
                    span (id = "save-status-text") : "Saved";
                    svg (class = "spinner", width = "15px", height = "15px", viewBox="0 0 66 66") {
                        circle (class="spinner-path", fill="none", stroke-width="6", stroke-linecap="round", cx="33", cy="33", r="30");
                    }
                }
            }

            link (rel = "stylesheet", type = "text/css", href = "https://cdn.jsdelivr.net/npm/summernote@0.8.16/dist/summernote-lite.min.css");

            script (src = "https://code.jquery.com/jquery-3.4.1.min.js");
            script (src = "https://cdn.jsdelivr.net/npm/summernote@0.8.18/dist/summernote-lite.min.js");

            script (src = asset_path("modules/api.js"));
            script (src = asset_path("modules/html.js"));
            script (src = asset_path("editor.js"));
        };
    }
}
