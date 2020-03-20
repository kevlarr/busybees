use horrorshow::{helper::doctype, html, RenderOnce, TemplateBuffer};

pub struct LayoutPage<C> {
    pub title: String,
    pub main_id: String,
    pub content: C,
}

impl<C> RenderOnce for LayoutPage<C>
where
    C: RenderOnce,
{
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let LayoutPage {
            title,
            main_id,
            content,
        } = self;

        tmpl << html! {
            : doctype::HTML;

            html {
                head {
                    title : format!("Busy Bee Life | {}", title);
                    meta(charset = "utf-8");

                    link(rel = "stylesheet", type = "text/css", href = "/public/assets/app.css");

                    // Font families
                    link(rel = "stylesheet", type = "text/css", href = "https://fonts.googleapis.com/css?family=Damion&display=swap");
                    link(rel = "stylesheet", type = "text/css", href = "https://fonts.googleapis.com/css?family=Work+Sans:300,300i,600&display=swap");
                    link(rel = "stylesheet", type = "text/css", href = "https://fonts.googleapis.com/css?family=Cormorant+Garamond:400&display=swap");
                }

                body {
                    main(id = main_id) : content;

                    nav {
                        a (id = "Header", href = "/") {
                            header : "The busy bee life";
                        }
                        ul (id = "AdminPages") {
                            li { a (href = "/posts/new", class = "admin page-link") : "New post"; }
                            li { a (href = "/drafts", class = "admin page-link") : "Drafts"; }
                        }
                        ul (id = "Pages") {
                            li { a (href = "/about", class = "page-link") : "About us"; }
                            li { a (href = "/sandbox", class = "page-link") : "Sandbox"; }
                        }
                        footer {
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
