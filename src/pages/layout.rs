use horrorshow::{
    helper::doctype,
    html,
    RenderOnce,
    TemplateBuffer,
};

pub struct Layout<C> {
    pub title: String,
    pub content: C,
}

impl<C> RenderOnce for Layout<C> where C: RenderOnce {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Layout { title, content } = self;

        tmpl << html! {
            : doctype::HTML;

            html {
                head {
                    title : title;
                }

                header {
                    p : "busy bee life";
                }

                body : content;

                footer {
                    p : "powered by Rust";
                }
            }
        };
    }
}
