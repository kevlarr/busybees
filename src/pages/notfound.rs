use crate::pages::Page;
use horrorshow::{html, RenderOnce, TemplateBuffer};

/// Async wrapper for use as a handler and page extractor
pub async fn get(page: Page) -> Page {
    get_sync(page)
}

/// Sync function to render the page itself
pub fn get_sync(page: Page) -> Page {
    page.id("NotFound")
        .title("Not Found")
        .content(NotFound{})
}

pub struct NotFound;

impl RenderOnce for NotFound {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1(class = "not-found-header") : "Wh... where am I..?";
            img(src = "/public/images/sad-bee-md.png", class = "not-found-logo");
        };
    }
}
