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
                    title : format!("Busy Bee Life | {}", title);
                    link(rel = "stylesheet", type = "text/css", href = "/public/app.css");

                    // Header font family
                    link(rel = "stylesheet", href = "https://fonts.googleapis.com/css?family=Damion&display=swap");

                    // Body font families
                    link(rel = "stylesheet", href = "https://fonts.googleapis.com/css?family=Cormorant+Garamond:600|Raleway:300&display=swap")
                }

                body {
                    header {
                        img(src = "/public/images/honeycomb1.png", class="logo-main");
                        span(class = "site-title") : "The busy bee life";
                    }

                    main : content;

                    footer {
                        p : "© 2020 | Powered by Rust, PostgreSQL, and 🐝s";
                    }
                }
            }
        };
    }
}
