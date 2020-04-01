use crate::{extensions::Assigns, models::AuthorWithoutPassword, State};

use actix_service::{Service, Transform};
use actix_session::{Session, UserSession};
use actix_web::{dev::{ServiceRequest, ServiceResponse}, web::{self, Data}, Error};
use futures::future::{ok, Ready, FutureExt};
use std::task::{Context, Poll};
use std::future::Future;

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
    S::Future: Future,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let auth = req.get_session().get::<i32>("auth");
        let data = req.app_data::<State>();

        async {
            self.service.call(req)
        }



        //match (auth, data) {
            //(Ok(Some(id)), Some(state)) => {
                //let pool = &mut *state.pool.borrow_mut();

                //sqlx::query_as!(
                    //AuthorWithoutPassword,
                    //"select id, email, name from author where id = $1",
                    //id,
                //).fetch_one(pool);

                //self.service.call(req)
            //},
            //_ => self.service.call(req),
        //}




            //if let (Ok(Some(id)), Some(state)) = (
                //req.get_session().get::<i32>("auth"),
                //req.app_data::<State>()
            //) {

                //let result = sqlx::query_as!(
                    //AuthorWithoutPassword,
                    //"select id, email, name from author where id = $1",
                    //id,
                //).fetch_one(pool).await;

                //let mut exts = req.head().extensions_mut();
                //let assigns = exts.get_mut::<Assigns>();

                //println!("ID: {}", id);
                //println!("AUTHOR: {:?}", result);
                //println!("ASSIGNS: {:?}", assigns);

                //if let (Some(Ok(author)), Some(assigns)) = (result, exts.get_mut::<Assigns>()) {
                    //assigns.user = Some(author);
                //}
            //}
        //}

        //self.service.call(req)
    }
}
