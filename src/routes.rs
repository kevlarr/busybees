use std::future::Future;

use actix_files::Files;
use actix_service::ServiceFactory;
use actix_web::{
    dev::{
        Factory,
        RequestHead,
        ServiceRequest,
        ServiceResponse,
    },
    guard::fn_guard,
    middleware::DefaultHeaders,
    web,
    FromRequest,
    Responder,
    Scope,
};

use crate::{
    actions as ax,
    guards::auth_guard,
    ASSET_BASEPATH,
    State,
};


struct Routes {
    service: Scope,
}

impl Routes {
    fn new() -> Self {
        Self {
            service: web::scope("/"),
        }
    }

    fn default<F, T, R, U>(mut self, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.service = self.service.default_service(web::route().to(handler));
        self
    }

    fn scope(mut self, path: &str, cb: impl Fn(Routes) -> Routes) -> Self {
        let routes = Routes { service: web::scope(path) };

        self.service = self.service.service(cb(routes).service);
        self
    }

    fn guard<F>(mut self, f: F) -> Self
    where
        F: Fn(&RequestHead) -> bool + 'static,
    {
        self.service = self.service.guard(fn_guard(f));
        self
    }

    fn get<F, T, R, U>(mut self, path: &str, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.service = self.service.route(path, web::get().to(handler));
        self
    }

    fn post<F, T, R, U>(mut self, path: &str, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.service = self.service.route(path, web::post().to(handler));
        self
    }

    fn patch<F, T, R, U>(mut self, path: &str, handler: F) -> Self
    where
        F: Factory<T, R, U>,
        T: FromRequest + 'static,
        R: Future<Output = U> + 'static,
        U: Responder + 'static,
    {
        self.service = self.service.route(path, web::patch().to(handler));
        self
    }

    fn map_files<F>(
        mut self,
        urlpath: &str,
        dirpath: &str,
        cb: impl Fn(Scope) -> Scope<F>
    ) -> Self
    where
        F: ServiceFactory<
            Config = (),
            Request = ServiceRequest,
            Response = ServiceResponse,
            Error = actix_web::Error,
            InitError = (),
        > + 'static,
    {
        let handler = Files::new("/", dirpath)
            .show_files_listing()
            .use_last_modified(true);

        let scope = web::scope(urlpath).service(handler);

        self.service = self.service.service(cb(scope));
        self
    }
}


pub fn from_state(s: &State) -> Scope {
    Routes::new()
        .map_files(&ASSET_BASEPATH, "www/assets", |files| files.wrap(
            DefaultHeaders::new().header("Cache-Control", "max-age=31536000")
        ))
        .map_files("/public", "www/public", |files| files.wrap(
            DefaultHeaders::new().header("Cache-Control", "max-age=31536000")
        ))
        .map_files("/uploads", &s.upload_path, |files| files.wrap(
            DefaultHeaders::new().header("Cache-Control", "max-age=31536000")
        ))
        .get("", ax::home)
        .get("/about", ax::about)
        .get("/sandbox", ax::sandbox)
        .scope("/admin", |admin| admin
            .guard(auth_guard)
            .scope("/posts", |posts| posts
                .get("/", ax::admin::posts::get)
                .get("/new", ax::admin::posts::new)
                .get("/edit/{key}", ax::admin::posts::edit)
                .get("/delete/{key}", ax::admin::posts::delete)
            )
            .get("/test", ax::about)
        )
        .scope("/api", |api| api
            .guard(auth_guard)
            .scope("/posts", |posts| posts
                .scope("/{key}", |post| post
                    .patch("/", ax::api::posts::update)
                    .get  ("/images", ax::api::posts::images::list)
                    .post ("/images/new", ax::api::posts::images::upload)
                    .patch("/published", ax::api::posts::update_published)
                )
            )
        )
        .scope("/auth", |auth| auth
            .get("", ax::auth::get)
            .post("", ax::auth::post)
            .get("/clear", ax::auth::delete)
        )
        .scope("/posts", |posts| posts
            .get("/{key}/read/{slug}", ax::posts::get_post)
        )
        .default(ax::not_found)
        .service
}
