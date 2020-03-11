use horrorshow::{helper::doctype, RenderOnce,  TemplateBuffer, Template};
use horrorshow::html;
use warp::Filter;


struct Page<C> {
    title: String,
    content: C,
}

impl<C> RenderOnce for Page<C> where C: RenderOnce {
    fn render_once(self, tmpl: &mut TemplateBuffer) {
        let Page { title, content } = self;

        tmpl << html! {
            : doctype::HTML;

            html {
                head {
                    title : title;
                }
                body : content
            }
        };
    }
}


#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| {
            let greeting = format!("Hello, {}!", name);

            let h1 = html!  {
                h1 : greeting.clone();
            };

            return warp::reply::html(Page {
                title: greeting.clone(),
                content: h1,
            }.into_string().unwrap());
        });


    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
