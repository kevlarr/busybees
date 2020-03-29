use super::{layout::Layout, Renderable};
use horrorshow::{html, RenderOnce, Template, TemplateBuffer};

pub struct AuthPage {
    error_message: Option<String>,
}

impl AuthPage {
    pub fn new() -> Self {
        AuthPage {
            error_message: None,
        }
    }

    pub fn with_error(msg: String) -> Self {
        AuthPage {
            error_message: Some(msg),
        }
    }
}

impl RenderOnce for AuthPage {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let AuthPage { error_message } = self;

        tmpl << html! {
            form (method = "post", action = "/auth") {
                h1 (id = "SignInWelcome") : "W";

                input (id = "SignInEmail",    name = "email",    type = "email",    placeholder = "Email", autofocus);
                input (id = "SignInPassword", name = "password", type = "password", placeholder = "Password");

                @ if let Some(msg) = error_message {
                    form-message (type = "error") : msg;
                }

                button (id = "SignInSubmit",  type = "submit", class = "primary", disabled) : "Sign In";
            }

            script(src = "/public/assets/signin.js");
        };
    }
}

impl Into<String> for AuthPage {
    fn into(self) -> String {
        Layout {
            title: Some("Sign In".into()),
            main_id: Some("Auth".into()),
            content: Some(self),
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating auth page".into())
    }
}

impl Renderable for AuthPage {}
