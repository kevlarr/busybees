use horrorshow::{html, Template};
use warp::Filter;

use ::busybees::{pages, Layout};


#[tokio::main]
async fn main() {
    let routes = warp::path("public")
        .and(warp::get())
        .and(warp::fs::dir("www/public"))

        .or(warp::path!("about")
            .and(warp::get())
            .map(|| warp::reply::html::<String>(
                pages::about::AboutPage.into()
            )))

        .or(warp::path!("hello" / String)
            .and(warp::get())
            .map(|name| {
                let greeting = format!("Hello, {}!", name);

                let h1 = html! {
                    h1 : greeting.clone();
                };

                return warp::reply::html(Layout {
                    title: greeting.clone(),
                    content: h1,
                }.into_string().unwrap());
            }));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
