use super::{layout::LayoutPage, Renderable};
use horrorshow::{html, RenderOnce, Template, TemplateBuffer};

pub struct SandboxPage;

impl RenderOnce for SandboxPage {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        tmpl << html! {
            h1 : "This is an h1 header";
            p  : "Ow, my spirit! Hey, what kinda party is this? There's no booze and only one hooker. Ok, we'll go deliver this crate like professionals, and then we'll go ride the bumper cars.";

            ul {
                li : "The first thing to do";
                li : "And the second thing to do";
                li : "Why don't I have real things here?";
            }

            h2 : "This is an h2 header";
            p  : "Professor, make a woman out of me. A true inspiration for the children. So, how 'bout them Knicks? WINDMILLS DO NOT WORK THAT WAY! GOOD NIGHT! Oh, but you can. But you may have to metaphorically make a deal with the devil. And by \"devil\", I mean Robot Devil. And by \"metaphorically\", I mean get your coat.";

            h3 : "This is an h3 header";
            p  : "Bite my shiny metal ass. Anyone who laughs is a communist! Kids have names? That's right, baby. I ain't your loverboy Flexo, the guy you love so much. You even love anyone pretending to be him! Alright, let's mafia things up a bit. Joey, burn down the ship. Clamps, burn down the crew.";

            h4 : "This is an h4 header";
            p  : "Yeah, and if you were the pope they'd be all, \"Straighten your pope hat.\" And \"Put on your good vestments.\" Who said that? SURE you can die! You want to die?! You know, I was God once. I'm sure those windmills will keep them cool.adf";

            h5 : "This is an h5 header";
            p  : "Well, let's just dump it in the sewer and say we delivered it. Yes, I saw. You were doing well, until everyone died. We can't compete with Mom! Her company is big and evil! Ours is small and neutral! Fry, we have a crate to deliver.";

            h6 : "This is an h6 header";
            p  : "Man, I'm sore all over. I feel like I just went ten rounds with mighty Thor. Large bet on myself in round one. I was all of history's great robot actors - Acting Unit 0.8; Thespomat; David Duchovny! Goodbye, cruel world. Goodbye, cruel lamp. Goodbye, cruel velvet drapes, lined with what would appear to be some sort of cruel muslin and the cute little pom-pom curtain pull cords. Cruel though they may be…";

        };
    }
}

impl Into<String> for SandboxPage {
    fn into(self) -> String {
        LayoutPage {
            title: "Sandbox".into(),
            main_id: "Sandbox".into(),
            content: self,
        }
        .into_string()
        .unwrap_or_else(|_| "There was an error generating sandbox page".into())
    }
}

impl Renderable for SandboxPage {}
