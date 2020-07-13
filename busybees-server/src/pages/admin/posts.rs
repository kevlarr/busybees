use busybees::{
    store::posts::{AdminPostMeta, PostDetail, PostLink},
    deps::sqlx::Error as SqlxError,
};
use crate::asset_path;
use horrorshow::{html, Raw, RenderOnce, TemplateBuffer};

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
        tmpl << html! {
            form(id = "editor-form", data-post-key = &self.post.key) {
                input(id = "post-title", name = "title", placeholder = "Title", autofocus = "true", value = &self.post.title);
                textarea(id = "summernote-editor", name = "content") : &self.post.content;

                fieldset {
                    legend : "Cover Image";
                    div(id = "post-images");
                }
            }

            div (id = "form-meta") {
                div (id = "save-status") {
                    span (id = "save-status-text") : "Saved";
                    svg (class = "spinner", width = "15px", height = "15px", viewBox="0 0 66 66") {
                        circle (class="spinner-path", fill="none", stroke-width="6", stroke-linecap="round", cx="33", cy="33", r="30");
                    }
                }
                a (id = "preview-link", href = self.post.href()) : Raw("View post &rarr;");
            }

            // WYSIWYG editor
            link (rel = "stylesheet", type = "text/css", href = "https://cdn.jsdelivr.net/npm/summernote@0.8.16/dist/summernote-lite.min.css");

            script (src = "https://code.jquery.com/jquery-3.4.1.min.js");
            script (src = "https://cdn.jsdelivr.net/npm/summernote@0.8.18/dist/summernote-lite.min.js");

            script (src = asset_path("modules/api.js"));
            script (src = asset_path("modules/html.js"));
            script (src = asset_path("editor.js"));
        };
    }
}
