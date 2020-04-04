use crate::{
    models::{Post, NewPost},
    pages::{notfound, Page},
    ActixResult,
    State,
    redirect,
};

use actix_web::{
    web::{Data, Form, Path},
    HttpResponse,
};
use chrono::Utc;
use horrorshow::{html, RenderOnce, TemplateBuffer};

pub struct PostForm {
    pub post: Option<Post>,
}

impl PostForm {
    pub async fn new(page: Page) -> Page {
        page.id("PostForm")
            .title("Create Post")
            .content(Self{ post: None })
    }

    pub async fn create(
        form: Form<NewPost>,
        state: Data<State>,
    ) -> ActixResult {
        let pool = &mut *state.pool.borrow_mut();
        let slug = slug::slugify(&form.title);

        match Post::create(pool, form.into_inner()).await {
            Ok(key) => Ok(redirect(&format!("/posts/{}/read/{}", key, slug))),
            Err(e) => Ok(HttpResponse::BadRequest().body(e.to_string())),
        }
    }

    pub async fn edit(
        page: Page,
        path: Path<(String,)>,
        state: Data<State>,
    ) -> Page {
        let pool = &mut *state.pool.borrow_mut();

        match Post::load(pool, path.0.clone()).await {
            Ok(post) => page
                .id("PostForm")
                .title("Edit Post")
                .content(Self{ post: Some(post) }),

            Err(_) => notfound::get_sync(page),
        }
    }

    pub async fn update(
        path: Path<(String,)>,
        form: Form<NewPost>,
        state: Data<State>,
    ) -> ActixResult {
        let pool = &mut *state.pool.borrow_mut();
        let slug = slug::slugify(&form.title);

        Ok(match Post::update(pool, path.0.clone(), form.into_inner()).await {
            Ok(_) => redirect(&format!("/posts/{}/read/{}", path.0, slug)),
            Err(e) => HttpResponse::BadRequest().body(e.to_string()),
        })
    }
}

impl RenderOnce for PostForm {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let (action, title, content) = match self.post {
            Some(Post {
                key,
                title,
                content,
                ..
            }) => (format!("/posts/{}/edit", key), title, content),
            None => ("/posts/new".to_string(), String::new(), String::new()),
        };

        tmpl << html! {
            form(id = "EditorForm", method = "post", action = action) {
                input(id = "PostTitle", name = "title", placeholder = "How I stopped the zombie apocalypse...", autofocus = "true", value = title);
                textarea(id = "SummernoteEditor", name = "content") : content;

                div(id = "PostControls") {
                    a (href = "/") { button(type = "button") : "Cancel"; }
                    button(id = "SubmitEditor", type = "submit", class = "primary", disabled) : "Submit";
                }
            }

            // WYSIWYG editor
            link(rel = "stylesheet", type = "text/css", href = "https://cdn.jsdelivr.net/npm/summernote@0.8.16/dist/summernote-lite.min.css");

            script(src = "https://code.jquery.com/jquery-3.4.1.min.js");
            script(src = "https://cdn.jsdelivr.net/npm/summernote@0.8.16/dist/summernote-lite.min.js");
            script(src = "/public/assets/editor.js");
        };
    }
}
