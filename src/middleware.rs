//! Server middleware
use crate::extensions::Assigns;
use crate::store::authors::AuthorWithoutPassword;

use actix_service::Service;
use actix_session::UserSession;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::Error;

/// Adds a new `Assigns` to the request extensions.
pub fn set_assigns<S, B>(req: ServiceRequest, service: &mut S) -> S::Future
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
{
    req.head().extensions_mut().insert(Assigns { author: None });
    service.call(req)
}

/// Inspects the request session for an `auth` cookie and, if present,
/// adds the user information into the request extension's `Assigns`.
pub fn load_user<S, B>(req: ServiceRequest, service: &mut S) -> S::Future
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
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
