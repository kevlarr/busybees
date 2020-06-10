use horrorshow::{html, RenderOnce, TemplateBuffer};
use sqlx::Error as SqlxError;

use crate::asset_path;
use crate::store::posts::{AdminPostPreview, Post, TitleSlug};

pub struct Posts {
    pub posts: Result<Vec<AdminPostPreview>, SqlxError>,
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

pub struct PostForm {
    pub post: Post,
}

impl RenderOnce for PostForm {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Post {
            key,
            title,
            content,
            ..
        } = self.post;

        tmpl << html! {
            form (id = "EditorForm", data-post-key = key) {
                input(id = "PostTitle", name = "title", placeholder = "Title", autofocus = "true", value = title);
                textarea(id = "SummernoteEditor", name = "content") : content;
            }

            p (id = "saveStatus") : "Saved";

            // WYSIWYG editor
            link (rel = "stylesheet", type = "text/css", href = "https://cdn.jsdelivr.net/npm/summernote@0.8.16/dist/summernote-lite.min.css");

            script (src = "https://code.jquery.com/jquery-3.4.1.min.js");
            script (src = "https://cdn.jsdelivr.net/npm/summernote@0.8.18/dist/summernote-lite.min.js");
            script (src = asset_path("editor.js"));
        };
    }
}
