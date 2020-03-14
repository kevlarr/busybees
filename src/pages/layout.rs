use horrorshow::{helper::doctype, html, RenderOnce, TemplateBuffer};

pub struct Layout<C> {
    pub title: String,
    pub content: C,
}

impl<C> RenderOnce for Layout<C>
where
    C: RenderOnce,
{
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Layout { title, content } = self;

        tmpl << html! {
            : doctype::HTML;

            html {
                head {
                    title : format!("Busy Bee Life | {}", title);
                    meta(charset = "utf-8");

                    link(rel = "stylesheet", type = "text/css", href = "/public/assets/app.css");

                    // Header font family
                    link(rel = "stylesheet", type = "text/css", href = "https://fonts.googleapis.com/css?family=Damion&display=swap");

                    // Body font families
                    link(rel = "stylesheet", type = "text/css", href = "https://fonts.googleapis.com/css?family=Cormorant+Garamond:600|Raleway:300&display=swap");

                    // WYSIWYG editor
                    link(rel = "stylesheet", type = "text/css", href = "https://cdn.jsdelivr.net/npm/summernote@0.8.16/dist/summernote-lite.min.css");
                }

                body {
                    header {
                        img(src = "/public/images/honeycomb1.png", class="logo-main");
                        span(class = "site-title") : "The busy bee life";
                    }

                    main {
                        div(id = "innerMain") : content;
                    }

                    footer {
                        p {
                            : "© 2020 | Powered by ";
                            a(href = "https://www.rust-lang.org/", target = "_blank") : "Rust";
                            : ", ";
                            a(href = "https://www.postgresql.org/", target = "_blank") : "PostgreSQL";
                            : ", ";
                            a(href = "https://summernote.org/", target = "_blank") : "Summernote";
                            : ", and us 🐝s!";
                        }
                    }
                }
            }
        };
    }
}
