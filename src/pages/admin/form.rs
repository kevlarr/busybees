use crate::{
    models::{Post, NewPost, TitleSlug},
    pages::{notfound, Page},
    ActixResult,
    State,
    asset_path,
    redirect,
};

use actix_web::{
    web::{Data, Form, Path},
    HttpResponse,
};
use horrorshow::{html, RenderOnce, TemplateBuffer};

pub struct PostForm {
    pub post: Post,
}

impl PostForm {
    pub async fn new(state: Data<State>) -> ActixResult {
        let new_post = NewPost {
            title: "New post".into(),
            content: None,
        };

        match Post::create(&state.pool, new_post).await {
            Ok(key) => Ok(redirect(&format!("/admin/posts/edit/{}", key))),
            Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
        }
    }

    pub async fn edit(
        page: Page,
        path: Path<(String,)>,
        state: Data<State>,
    ) -> Page {
        match Post::load(&state.pool, path.0.clone()).await {
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

    pub async fn update(
        path: Path<(String,)>,
        form: Form<NewPost>,
        state: Data<State>,
    ) -> ActixResult {
        let slug = form.title_slug();

        Ok(match Post::update(&state.pool, path.0.clone(), form.into_inner()).await {
            Ok(_) => redirect(&format!("/posts/{}/read/{}", path.0, slug)),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        })
    }
}

impl RenderOnce for PostForm {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Post { key, title, content, .. } = self.post;
        let action = format!("/admin/posts/edit/{}", key);

        tmpl << html! {
            form(id = "EditorForm", method = "post", action = action) {
                input(id = "PostTitle", name = "title", placeholder = "How I stopped the zombie apocalypse...", autofocus = "true", value = title);
                textarea(id = "SummernoteEditor", name = "content") : content;

                div(id = "PostControls") {
                    a (href = "/") { button(type = "button") : "Cancel"; }
                    button(id = "SubmitEditor", type = "submit", class = "primary") : "Submit";
                }
            }

            // WYSIWYG editor
            link(rel = "stylesheet", type = "text/css", href = "https://cdn.jsdelivr.net/npm/summernote@0.8.16/dist/summernote-lite.min.css");

            script(src = "https://code.jquery.com/jquery-3.4.1.min.js");
            script(src = "https://cdn.jsdelivr.net/npm/summernote@0.8.16/dist/summernote-lite.min.js");
            script(src = asset_path("editor.js"));
        };
    }
}
