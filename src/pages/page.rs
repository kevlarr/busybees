use crate::{extensions::Assigns, models::AuthorWithoutPassword, asset_path};

use actix_web::{dev::Payload, Error, FromRequest, HttpRequest, HttpResponse, Responder};
use futures::future::{ok, Ready};
use horrorshow::{helper::doctype, html, Raw, RenderOnce, Template, TemplateBuffer};

pub struct Page {
    pub user: Option<AuthorWithoutPassword>,
    content: Option<String>,
    main_id: Option<String>,
    og_image: Option<String>,
    title: Option<String>,
    url: String,
}

impl Page {
    pub fn new(url: String, user: Option<AuthorWithoutPassword>) -> Self {
        Page {
            url,
            user,
            content: None,
            main_id: None,
            og_image: None,
            title: None,
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

    pub fn image(mut self, image: Option<impl Into<String>>) -> Self {
        if let Some(i) = image {
            self.og_image = Some(i.into());
        }
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
        let user = req.extensions()
            .get::<Assigns>()
            .map(|a| a.author.clone())
            .flatten();

        ok(Page::new(req.uri().to_string(), user))
    }
}

impl RenderOnce for Page {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Page {
            content,
            main_id,
            og_image,
            title,
            url,
            user,
        } = self;

        let title = match title {
            Some(t) => format!("Busy Bee Life | {}", t),
            None => "Busy Bee Life".to_string(),
        };

        let image = og_image.unwrap_or_else(|| "/public/images/canoe-crop.jpg".into());

        tmpl << html! {
            : doctype::HTML;

            html(lang = "en") {
                head {
                    title : &title;

                    meta(charset = "utf-8");
                    meta(name = "viewport", content = "width=device-width, initial-scale=1");

                    // Favicons
                    link(rel = "icon", href = "/public/images/favicon-16.png", type = "image/png", sizes = "16x16");
                    link(rel = "icon", href = "/public/images/favicon-32.png", type = "image/png", sizes = "32x32");
                    link(rel = "apple-touch-icon", href="/public/images/apple-touch-icon.png", type = "image/png", sizes = "180x180");

                    // Object Graph properties
                    meta(property = "og:type", content = "website");
                    meta(property = "og:image", content = &image);
                    meta(property = "og:site_name ", content = "Busy Bee Life");
                    meta(property = "og:title", content = &title);
                    meta(property = "og:url", content = &url);

                    // Application assets
                    link(rel = "stylesheet", type = "text/css", href = asset_path("app.css"));

                    // Font families
                    link(rel = "stylesheet", type = "text/css", href = "https://fonts.googleapis.com/css?family=Dancing+Script:wght@600&display=swap");
                    link(rel = "stylesheet", type = "text/css", href = "https://fonts.googleapis.com/css?family=Work+Sans:300,300i,600&display=swap");
                    link(rel = "stylesheet", type = "text/css", href = "https://fonts.googleapis.com/css?family=Cormorant+Garamond:400&display=swap");

                    // Font Awesome assets are ~80kb - not huge but no reason to include for non-admins
                    @ if let Some(_) = user {
                        script (src = "https://use.fontawesome.com/195e7e8d92.js");
                    }
                }

                body {
                    main-nav {
                        header {
                            logotype {
                                a (href = "/") {
                                    img (id = "Logo", src = "/public/images/b-logo-white.svg");
                                    : "usy bee life";
                                }
                            }

                            @ if let Some(_) = user {
                                ul (id = "AdminLinks") {
                                    li {
                                        a (class = "icon-link", href = "/admin/posts") {
                                            i (class = "fa fa-th-list");
                                            : " Manage Posts";
                                        }
                                    }
                                    li {
                                        a (class = "icon-link", href = "/admin/posts/new") {
                                            i (class = "fa fa-file-text-o");
                                            : " New Post";
                                        }
                                    }
                                    li {
                                        a (class = "icon-link", href = "/auth/clear") {
                                            i (class = "fa fa-lock");
                                            : " Sign Out";
                                        }
                                    }
                                }
                            }
                            bio {
                                snapshot {
                                    img (src = "/public/images/pose-crop.jpg");
                                    dl {
                                        div { dt : "Stacey"; dd : "Attorney"; }
                                        div { dt : "Kevin"; dd : "Software Engineer"; }
                                    }
                                }
                                summary {
                                    small-display {
                                        strong: "Stacey";
                                        : " (attorney) and ";
                                        strong: "Kevin";
                                        : " (software engineer). "
                                    }
                                    : "Parents, DIY home-renovators, budding environmentalists, and all around busy bees. ";
                                    a (class = "basic", href = "/about") : "More about us ➝";
                                }
                            }
                        }

                        section {
                            // Author links, tags, etc.
                        }

                        footer {
                            :"Powered by ";
                            a (class = "basic", href = "https://www.rust-lang.org/", target = "_blank", rel = "noopener") : "Rust";
                            : ", ";
                            a (class = "basic", href = "https://www.postgresql.org/", target = "_blank", rel = "noopener") : "PostgreSQL";
                            : ", and ";
                            a (class = "basic", href = "https://summernote.org/", target = "_blank", rel = "noopener") : "Summernote";
                            : " © 2020";
                        }
                    }

                    main(id = main_id) : Raw(if let Some(c) = content { c } else { String::new() });
                }
            }
        };
    }
}
