use crate::{extensions::Assigns, models::AuthorWithoutPassword};

use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, HttpResponse, Responder};
use futures::future::{ok, Ready};
use horrorshow::{helper::doctype, html, Raw, RenderOnce, Template, TemplateBuffer};

pub struct Page {
    pub user: Option<AuthorWithoutPassword>,
    title: Option<String>,
    main_id: Option<String>,
    content: Option<String>,
}

impl Page {
    pub fn new(user: Option<AuthorWithoutPassword>) -> Self {
        Page {
            user,
            title: None,
            main_id: None,
            content: None,
        }
    }

    pub fn content(mut self, content: impl RenderOnce) -> Self {
        self.content = Some(content
            .into_string()
            .unwrap_or_else(|e| e.to_string())
        );
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.main_id = Some(id.into());
        self
    }
}

impl Responder for Page {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _: &HttpRequest) -> Self::Future {
        ok(HttpResponse::Ok().body(self.into_string().unwrap_or_else(|e| e.to_string())))
    }
}

impl FromRequest for Page {
    type Config = ();
    type Error = Error;
    type Future = Ready<Result<Page, Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let user = req.extensions().get::<Assigns>().map(|a| a.author.clone()).flatten();

        ok(Page::new(user))
    }
}

impl RenderOnce for Page {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Page {
            title,
            main_id,
            content,
            user,
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
                    main(id = main_id) : Raw(if let Some(c) = content { c } else { String::new() });

                    main-nav {
                        header {
                            a (id = "Logotype", href = "/") : "busy bee life";

                            @ if let Some(_) = user {
                                ul (id = "AdminLinks") {
                                    li { a (class = "icon-link", href = "/admin/posts") : "⌸ Manage Posts"; }
                                    li { a (class = "icon-link", href = "/admin/posts/new") : "⎘ New Post"; }
                                    li { a (class = "icon-link", href = "/auth/clear") : "⍇ Sign Out"; }
                                }
                            }

                            bio {
                                img (src = "/public/images/pose-crop.jpg");

                                dl {
                                    div {
                                        dt : "Stacey";
                                        dd : "Attorney";
                                    }
                                    div {
                                        dt : "Kevin";
                                        dd : "Software Engineer";
                                    }
                                }
                            }

                            p {
                                : "Parents, DIY home-renovators, budding environmentalists, and all-around busy bees. ";
                                a (href = "/about") : "More about us ➝";
                            }
                        }

                        section {
                            // Author links, tags, etc.
                        }

                        footer {
                            p {
                                :"Powered by ";
                                a (href = "https://www.rust-lang.org/", target = "_blank") : "Rust";
                                : " and ";
                                a (href = "https://www.postgresql.org/", target = "_blank") : "PostgreSQL";
                                : " © 2020";
                            }
                        }
                    }
                }
            }
        };
    }
}
