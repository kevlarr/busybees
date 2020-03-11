use horrorshow::{
    html,
    RenderOnce,
    TemplateBuffer,
    Template,
};
use super::layout::Layout;

pub struct AboutPage;

impl RenderOnce for AboutPage {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1 : "About us"
        };
    }
}

impl Into<String> for AboutPage {
    fn into(self) -> String {
        Layout {
            title: "About us busy bees".into(),
            content: self,
        }.into_string().unwrap()
    }
}
