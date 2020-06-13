use horrorshow::{html, RenderOnce, TemplateBuffer};

pub struct NotFound;

impl RenderOnce for NotFound {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1(class = "not-found-header") : "Wh... where am I..?";
            img(src = "/public/images/sad-bee-md.png", class = "not-found-logo");
        };
    }
}
