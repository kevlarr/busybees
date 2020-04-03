use crate::{
    extensions::Assigns,
    models::{AuthorWithoutPassword, Post, NewPost},
    pages::{notfound, Page},
    ActixResult,
    State,
    redirect,
};

use actix_web::{
    dev::RequestHead,
    guard::fn_guard,
    web::{self, Data, Form, Path},
    HttpResponse,
    Scope,
};
use chrono::Utc;
use horrorshow::{html, RenderOnce, TemplateBuffer};


fn auth_guard(head: &RequestHead) -> bool {
    let author: Option<AuthorWithoutPassword> = head
        .extensions()
        .get::<Assigns>()
        .map(|assn| assn.author.clone())
        .flatten();

    author.is_some()
}


pub fn resource(path: &str) -> Scope {
    use web::{get, post};

    web::scope(path)
        .guard(fn_guard(auth_guard))
        .route("/new", get().to(PostForm::new))
        .route("/new", post().to(PostForm::create))
        .route("/{key}/edit", get().to(PostForm::edit))
        .route("/{key}/edit", post().to(PostForm::update))
}

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
        let now = Utc::now();

        let result = sqlx::query!(
            "insert into post (title, content, published, created_at, updated_at)
                values ($1, $2, $3, $4, $5) returning key",
            form.title,
            form.content,
            false,
            now,
            now
        )
        .fetch_one(pool)
        .await;

        match result {
            Ok(row) => {
                let slug = slug::slugify(&form.title);
                Ok(redirect(&format!("/posts/{}/read/{}", row.key, slug)))
            }
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

        let result = sqlx::query!(
            "update post set title = $1, content = $2, updated_at = now() where key = $3",
            form.title,
            form.content,
            path.0
        )
        .execute(pool)
        .await;

        Ok(match result {
            Ok(_) => redirect_to_post(&path.0, &form.title),
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


fn redirect_to_post(key: &str, title: &str) -> HttpResponse {
    let slug = slug::slugify(title);
    redirect(&format!("/posts/{}/read/{}", key, slug))
}
