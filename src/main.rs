use chrono::Utc;
use warp::Filter;

use ::busybees::pages;


#[tokio::main]
async fn main() {
    let routes =
        warp::path("public")
            .and(warp::get())
            .and(warp::fs::dir("www/public"))

        .or(warp::path!("sandbox")
            .and(warp::get())
            .map(|| warp::reply::html::<String>(
                pages::Sandbox.into()
            )))

        .or(warp::path!("about")
            .and(warp::get())
            .map(|| warp::reply::html::<String>(
                pages::About.into()
            )))

        .or(warp::path!("articles" / String)
            .and(warp::get())
            .map(|title: String| {

                let now = Utc::now();
                let post = pages::Post {
                    title: title.clone(),
                    body: "<p style='color: red'>some content</p>".into(),
                    created_at: now.clone(),
                    updated_at: now,
                };

                return warp::reply::html::<String>(post.into());
            }));

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
