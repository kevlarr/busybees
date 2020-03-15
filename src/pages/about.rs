use super::layout::Layout;
use horrorshow::{html, RenderOnce, Template, TemplateBuffer};

pub struct About;

/*
 * We're Kevin and Stacey. We are parents of a toddler in New England, a practicing attorny, a
 * software engineer, home renovators, eco-conscious fanatics, and all around busy bees.
 *
 * We have spent decades changing and evolving to be the best versions of ourselves,
 * which is how we found each other. It hasn't been easy, and might be why we never. stop. moving.
 * But it suits us well! We hope you enjoy this journey as much as we do!
 */

impl RenderOnce for About {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1 : "Hey there!";
            p : "Stacey and Kevin of New England, here.";
            p : "We are a practicing attorney, a software engineer, parents of a (curious, independent, and, ahem, spirited) toddler, renovators of our nearly 200-year-old new-to-us home, increasingly eco-conscious fanatics, and all around busy bees.";
            p {
                : "We have each spent decades working, evolving, and often struggling to be better versions of ourselves, which is the only way we were able to find each other - and become such a strong team. It hasn't been easy.  It might also be why we (and our toddler) ";
                em : "never. stop. moving...";
            }
            p : "But it works for us and apparently suits us well. This blog is our next journey, and we hope you'll enjoy it as much as we will!";

        };
    }
}

impl Into<String> for About {
    fn into(self) -> String {
        Layout {
            title: "About us busy bees".into(),
            main_id: "About".into(),
            content: self,
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating about page".into())
    }
}
