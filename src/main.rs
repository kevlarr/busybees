use actix_web::{App, HttpResponse, HttpServer, Responder, get};

use ::busybees::pages;


#[get("/sandbox")]
async fn sandbox() -> impl Responder {
    HttpResponse::Ok().body::<String>(pages::Sandbox.into())
}



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                actix_files::Files::new("/public", "www/public")
                    .show_files_listing()
                    .use_last_modified(true),
            )
            .service(sandbox)
    })
    .bind("127.0.0.1:3030")?
    .run()
    .await
}
