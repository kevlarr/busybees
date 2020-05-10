use crate::State;
use crate::extensions::Assigns;
use crate::models::AuthorWithoutPassword;

use actix_service::{Service, Transform};
use actix_session::UserSession;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;
use futures::future::{ok, Ready};
use std::cell::RefCell;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};


/// Adds a new `Assigns` struct to the request extensions.
pub fn set_assigns<S, B>(req: ServiceRequest, service: &mut S) -> S::Future
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
{
    req.head().extensions_mut().insert(Assigns{ author: None });
    service.call(req)
}


/// Inspects the request session for an `auth` cookie and, if present,
/// adds the user information into the request extensions' assigns.
pub fn load_user<S, B>(req: ServiceRequest, service: &mut S) -> S::Future
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
{
    {
        let auth = req.get_session().get::<AuthorWithoutPassword>("auth");
        let mut exts = req.head().extensions_mut();
        let assigns = exts.get_mut::<Assigns>();

        if let (Ok(auth), Some(assigns)) = (auth, assigns) {
            assigns.author = auth;
        }
    }

    service.call(req)
}


/// Service responsible for creating a new RequestLoggerMiddleware.
pub struct RequestLogger;

impl<S: 'static, B> Transform<S> for RequestLogger
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestLoggerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(RequestLoggerMiddleware {
            service: Rc::new(RefCell::new(service)),
        })
    }
}

pub struct RequestLoggerMiddleware<S> {
    // This is special: We need this to avoid lifetime issues.
    service: Rc<RefCell<S>>,
}

impl<S, B> Service for RequestLoggerMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    /// Logs specific request details to the database prior to calling next service.
    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let mut svc = self.service.clone();

        Box::pin(async move {
            if let Some(state) = req.app_data::<State>() {
                let pathname = req.path().to_string();
                let info = req.connection_info();
                let ip = info.remote();

                let referer = req.headers().get("referer")
                    .map(|r| r.to_str().ok())
                    .flatten();

                let _ = sqlx::query!(
                    "insert into page_view (pathname, ip, referer) values ($1, $2, $3)",
                    pathname, ip, referer,
                ).execute(&state.pool).await;
            }

            Ok(svc.call(req).await?)
        })
    }
}
