use crate::{
    pages::Page,
    asset_path,
};
use horrorshow::{html, RenderOnce, TemplateBuffer};

pub struct Auth {
    error_message: Option<String>,
}

impl Auth {
    pub fn new() -> Self {
        Auth { error_message: None }
    }

    pub fn with_error(msg: impl Into<String>) -> Self {
        Auth {
            error_message: Some(msg.into()),
        }
    }

    pub fn in_page(self, page: Page) -> Page {
        page.title("Sign In")
            .id("Auth")
            .content(self)
    }
}

impl RenderOnce for Auth {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Auth { error_message } = self;

        tmpl << html! {
            form (method = "post", action = "/auth") {
                h1 (id = "SignInWelcome") : "W";

                input (id = "SignInEmail",    name = "email",    type = "email",    placeholder = "Email", autofocus);
                input (id = "SignInPassword", name = "password", type = "password", placeholder = "Password");

                @ if let Some(msg) = error_message {
                    form-message (type = "error") : msg;
                }

                button (id = "SignInSubmit", type = "submit", disabled) : "Sign In";
            }

            script(src = asset_path("signin.js"));
        };
    }
}
