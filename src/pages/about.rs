use super::layout::Layout;
use horrorshow::{html, RenderOnce, Template, TemplateBuffer};

pub struct About;

impl RenderOnce for About {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1 : "About us";
            p : "This is just a test page to play with templating";
        };
    }
}

impl Into<String> for About {
    fn into(self) -> String {
        Layout {
            title: "About Us".into(),
            content: self,
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating about page".into())
    }
}
