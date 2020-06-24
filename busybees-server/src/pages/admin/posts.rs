use busybees::{
    store::{
        images::PostImage,
        posts::{AdminPostMeta, PostDetail, TitleSlug},
    },
    deps::sqlx::Error as SqlxError,
};
use crate::{asset_path, upload_path};
use horrorshow::{html, RenderOnce, TemplateBuffer};

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

                a (class = "admin-post-title", href = format!("/posts/{}/read/{}", &post.key, post.title_slug())) {
                    h2 : &post.title;
                }
            }
        }
    }
}

pub struct PostForm {
    pub post: PostDetail,
    pub images: Vec<PostImage>,
}

impl RenderOnce for PostForm {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let PostDetail {
            key,
            title,
            content,
            ..
        } = self.post;
        let images = self.images;

        tmpl << html! {
            form(id = "editor-form", data-post-key = key) {
                input(id = "post-title", name = "title", placeholder = "Title", autofocus = "true", value = title);
                textarea(id = "summernote-editor", name = "content") : content;

                @ if images.len() > 0 {
                    label: "Select a preview image for the post";

                    ul(id = "post-images") {
                        @ for image in images {
                            li {
                                input(
                                    type = "radio",
                                    name = "previewImageId",
                                    id = format!("preview-image-{}", image.image_id),
                                    value = image.image_id,
                                    checked? = image.is_preview,
                                    hidden
                                );
                                label(for = format!("preview-image-{}", image.image_id)) {
                                    img(
                                        class = if image.is_preview {"post-image is-preview"} else {"post-image"},
                                        src = upload_path(&image.filename)
                                    )
                                }
                            }
                        }
                    }
                }
            }

            p (id = "save-status") : "Saved";

            // WYSIWYG editor
            link (rel = "stylesheet", type = "text/css", href = "https://cdn.jsdelivr.net/npm/summernote@0.8.16/dist/summernote-lite.min.css");

            script (src = "https://code.jquery.com/jquery-3.4.1.min.js");
            script (src = "https://cdn.jsdelivr.net/npm/summernote@0.8.18/dist/summernote-lite.min.js");
            script (src = asset_path("editor.js"));
        };
    }
}
