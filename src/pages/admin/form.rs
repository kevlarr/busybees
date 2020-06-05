use crate::{
    pages::{notfound, Page},
    store::posts::{self, Post, PostParams},
    ActixResult,
    State,
    asset_path,
    redirect,
};

use actix_web::{
    web::{Data, Path},
    HttpResponse,
};
use horrorshow::{html, RenderOnce, TemplateBuffer};

pub struct PostForm {
    pub post: Post,
}

impl PostForm {
    pub async fn new(state: Data<State>) -> ActixResult {
        let new_post = PostParams {
            title: "New post".into(),
            content: String::new(),
        };

        match posts::create(&state.pool, new_post).await {
            Ok(key) => Ok(redirect(&format!("/admin/posts/edit/{}", key))),
            Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
        }
    }

    pub async fn edit(
        page: Page,
        path: Path<(String,)>,
        state: Data<State>,
    ) -> Page {
        match posts::find(&state.pool, path.0.clone()).await {
            Ok(post) => page
                .id("PostForm")
                .title("Edit Post")
                .content(Self{ post }),

            Err(e) => {
                eprintln!("{}", e.to_string());
                notfound::get_sync(page)
            },
        }
    }
}

impl RenderOnce for PostForm {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Post { key, title, content, .. } = self.post;

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
