use crate::{extensions::Assigns, models::AuthorWithoutPassword};

use actix_service::Service;
use actix_session::UserSession;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;


pub fn set_assigns<S, B>(req: ServiceRequest, service: &mut S) -> S::Future
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
{
    req.head().extensions_mut().insert(Assigns{ user: None });
    service.call(req)
}


pub fn load_user<S, B>(req: ServiceRequest, service: &mut S) -> S::Future
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>
{
    if let Ok(Some(author)) = req.get_session().get::<AuthorWithoutPassword>("auth") {
        if let Some(assigns) = req.head().extensions_mut().get_mut::<Assigns>() {
            assigns.user = Some(author);
        }
    }

    service.call(req)
}
