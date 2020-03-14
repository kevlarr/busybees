use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::env;
use warp::{filters::multipart, Filter};
use futures::{TryFutureExt, TryStreamExt};
use ::busybees::pages;


#[derive(Deserialize, Serialize)]
struct ImageUploadData {
    baseurl: String,
    error: String,
    files: Vec<String>,
    message: String,
    path: String,
}

#[derive(Deserialize, Serialize)]
struct ImageUpload {
    success: bool,
    data: Option<ImageUploadData>,
}


impl ImageUpload {
    fn new() -> Self {
        ImageUpload {
            success: true,
            data: Some(ImageUploadData {
                //baseurl: "media.tenor.com/images/45b40812ca80c7ead48046e317283aff/".into(),
                baseurl: "http://localhost:3030/public/45b40812ca80c7ead48046e317283aff/".into(),
                error: String::new(),
                files: vec!["tenor.gif".into()],
                message: String::new(),
                //path: "media.tenor.com/images/45b40812ca80c7ead48046e317283aff/tenor.gif".into(),
                path: "http://localhost:3030/public/45b40812ca80c7ead48046e317283aff/tenor.gif".into(),
            }),
        }
    }
}


#[tokio::main]
async fn main() {
    // Set `RUST_LOG=todos=debug` to see debug logs, this only shows access logs.
    if env::var_os("RUST_LOG").is_none() {
        env::set_var("RUST_LOG", "busybees=info");
    }
    pretty_env_logger::init();

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

        .or(warp::path!("posts" / "new")
            .and(warp::get())
            .map(|| warp::reply::html::<String>(pages::NewPost.into())))

        .or(warp::path!("posts" / String)
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
            }))

        .or(warp::path!("api" / "upload" / "image")
            .and(warp::post())
            .and(multipart::form().max_length(5 * 1024 * 1024))
            .map(|_form: multipart::FormData| {
                return warp::reply::json(&ImageUpload::new());
            }));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "PATCH", "DELETE"]);

    let logger = warp::log("busybees");

    warp::serve(routes.with(logger).with(cors))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
