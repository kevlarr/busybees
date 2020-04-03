use crate::{extensions::Assigns, models::AuthorWithoutPassword};

use actix_service::Service;
use actix_session::UserSession;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;


pub fn set_assigns<S, B>(req: ServiceRequest, service: &mut S) -> S::Future
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
{
    req.head().extensions_mut().insert(Assigns{ author: None });
    service.call(req)
}


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
