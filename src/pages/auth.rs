use super::{layout::LayoutPage, Renderable};
use horrorshow::{html, RenderOnce, Template, TemplateBuffer};

pub struct AuthPage;

impl RenderOnce for AuthPage {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            form (method = "post", action = "/sign-in") {
                h1 (id = "SignInWelcome") : "W";

                input (id = "SignInEmail",    name = "email",    type = "email",    placeholder = "Email", autofocus);
                input (id = "SignInPassword", name = "password", type = "password", placeholder = "Password");
                button (id = "SignInSubmit",  type = "submit", class = "primary", disabled) : "Sign In";
            }

            script(src = "/public/assets/signin.js");
        };
    }
}

impl Into<String> for AuthPage {
    fn into(self) -> String {
        LayoutPage {
            title: "Sign In".into(),
            main_id: "Auth".into(),
            content: self,
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating auth page".into())
    }
}

impl Renderable for AuthPage {}
