use crate::{extensions::Assigns, models::AuthorWithoutPassword, State};

use actix_service::{Service, Transform};
use actix_session::Session;
use actix_web::{dev::{ServiceRequest, ServiceResponse}, web::Data, FromRequest, Error};
use futures::future::{ok, Ready, FutureExt};
use std::task::{Context, Poll};

pub struct LoadUser;

impl<S, B> Transform<S> for LoadUser
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoadUserMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoadUserMiddleware { service })
    }
}

pub struct LoadUserMiddleware<S> {
    service: S,
}

impl<S, B> Service for LoadUserMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let exts = req.head().extensions();

        let author_id: Option<i32> = exts.get::<Session>()
            .map(|session| session.get("auth").ok())
            .flatten()
            .flatten();

        if let (Some(id), Some(state)) = (author_id, req.app_data::<Data<State>>()) {
            let pool = &mut *state.pool.borrow_mut();
            let result = sqlx::query_as!(
                AuthorWithoutPassword,
                "select id, email, name from author where id = $1",
                id,
            ).fetch_one(pool).now_or_never();

            if let (Some(Ok(author)), Some(assigns)) = (result, exts.get::<Assigns>()) {
                assigns.user = Some(author);
            }
        }

        self.service.call(req)
    }
}
