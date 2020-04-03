use crate::{
    models::Post,
    pages::{notfound, Page},
    State,
};

use actix_web::{
    web::{self, Data, Path},
    Scope,
};
use horrorshow::{html, Raw, RenderOnce, TemplateBuffer};


pub fn resource(path: &str) -> Scope {
    web::scope(path)
        .route("/{key}/read/{slug}", web::get().to(PostView::get))
}


pub struct PostView {
    pub post: Post,
}

impl PostView {
    pub async fn get(
        page: Page,
        path: Path<(String, String)>,
        state: Data<State>,
    ) -> Page {
        let pool = &mut *state.pool.borrow_mut();

        match Post::load(pool, path.0.clone()).await {
            Ok(post) => page.id("Post")
                .title(post.title.clone())
                .content(Self{ post }),

            Err(_) => notfound::get_sync(page),
        }
    }
}

impl RenderOnce for PostView {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Post {
            title,
            content,
            published,
            created_at,
            updated_at,
            ..
        } = self.post;

        tmpl << html! {
            h1 : title;
            p : Raw(content);
            p : format!("Published: {}", published);
            p : format!("Created: {}", created_at);
            p : format!("Updated: {}", updated_at);
        };
    }
}
