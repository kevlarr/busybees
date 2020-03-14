use super::layout::Layout;
use horrorshow::{html, RenderOnce, Template, TemplateBuffer};

pub struct NotFound;

impl RenderOnce for NotFound {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1(class = "not-found-header") : "Wh... where am I..?";
            img(src = "/public/images/sad-bee-md.png", class = "not-found-logo");
        };
    }
}

impl Into<String> for NotFound {
    fn into(self) -> String {
        Layout {
            title: "There's nothing here".into(),
            main_id: "NotFound".into(),
            content: self,
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating 404 page".into())
    }
}
