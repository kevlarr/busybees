use super::{layout::Layout, Renderable};
use actix_web::{error::Error, HttpRequest, HttpResponse, Responder};
use futures::future::{self, Ready};
use horrorshow::{html, RenderOnce, Template, TemplateBuffer};

pub struct AboutPage;

impl RenderOnce for AboutPage {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1 : "Hey there!";
            p : "Stacey and Kevin of New England, here.";

            p : "We are a practicing attorney, a software engineer, parents of a (curious, independent,
                 and, ahem, spirited) toddler, renovators of our nearly 200-year-old new-to-us home,
                 increasingly eco-conscious fanatics, and all around busy bees.";
            p {
                : "We have each spent decades working, evolving, and often struggling
                   to be better versions of ourselves, which is the only way we were
                   able to find each other - and become such a strong team.
                   It hasn't been easy.
                   It might also be why we (and our toddler) ";
                em : "never. stop. moving...";
            }
            p : "But it works for us and apparently suits us well.
                This blog is our next journey, and we hope you'll enjoy it
                as much as we will!";

        };
    }
}

impl Into<String> for AboutPage {
    fn into(self) -> String {
        Layout {
            title: Some("About us busy bees".into()),
            main_id: Some("About".into()),
            content: Some(self),
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating about page".into())
    }
}

impl Renderable for AboutPage {}
