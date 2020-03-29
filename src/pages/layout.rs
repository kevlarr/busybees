use actix_web::{dev::Payload, Error, FromRequest, HttpRequest};
use futures::future::{Ready, ready};
use horrorshow::{helper::doctype, html, RenderOnce, TemplateBuffer};

pub struct Layout<C> {
    pub title: Option<String>,
    pub main_id: Option<String>,
    pub content: Option<C>,
}

impl<C> Layout<C> {
    pub fn new() -> Self {
        Layout {
            title: None,
            main_id: None,
            content: None,
        }
    }
}

impl<C> FromRequest for Layout<C> {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<Layout<C>, Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        ready(Ok(Layout::new()))
    }
}

impl<C> RenderOnce for Layout<C>
where
    C: RenderOnce,
{
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Layout {
            title,
            main_id,
            content,
        } = self;

        tmpl << html! {
            : doctype::HTML;

            html {
                head {
                    title : match title {
                        Some(t) => format!("The Busy Bee Life | {}", t),
                        None => "The Busy Bee Life".to_string(),
                    };

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
                        a (id = "Logotype", href = "/") : "The busy bee life";

                        ul (id = "AdminLinks") {
                            li { a (href = "/posts/new", class = "admin page-link") : "New post"; }
                            li { a (href = "/drafts", class = "admin page-link") : "Drafts"; }
                        }

                        ul (id = "Pages") {
                            li { a (href = "/about", class = "page-link") : "About us"; }
                            li { a (href = "/sandbox", class = "page-link") : "Sandbox"; }
                        }

                        footer {
                            p : "© 2020";
                            p {
                                :"Powered by ";
                                a(href = "https://www.rust-lang.org/", target = "_blank") : "Rust";
                                : ", ";
                                a(href = "https://www.postgresql.org/", target = "_blank") : "PostgreSQL";
                                : ", ";
                                a(href = "https://summernote.org/", target = "_blank") : "Summernote";
                                : ", and us 🐝s!";
                            }
                            p {
                                : "Images courtesy of ...";
                            }
                        }
                    }
                }
            }
        };
    }
}
