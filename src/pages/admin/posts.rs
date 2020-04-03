use crate::{
    models::Post,
    pages::Page,
    State,
};

use actix_web::web::Data;
use horrorshow::{html, RenderOnce, TemplateBuffer};

pub struct Posts {
    pub posts: Vec<Posts>,
}

impl Posts {
    pub async fn get(page: Page, state: Data<State>) -> Page {
        page.id("Posts").title("Posts").content(Self { posts: vec![] })
    }

}

impl RenderOnce for Posts {
    fn render_once(self, tmpl: &mut TemplateBuffer) {

        tmpl << html! {
            p : "Some posts here";
        };
    }
}
