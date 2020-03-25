//use actix_web::Json;
//use actix_web::middleware::session::RequestSession;

//use crate::{ApiResult, Request, Response};
//use crate::encryption;
//use crate::error::ApiError;
//use crate::stores::users;

//#[derive(Deserialize)]
//pub struct PostParams {
    //email: String,
    //password: String,
//}

//pub fn create((req, params): (Request, Json<PostParams>)) -> ApiResult<Response> {
    //let secret = &req.state().secret_key;

    //let user = match users::find_by_email(&req.state().conn, &params.email)? {
        //Some(u) => u,
        //None => {
            //// Hash the password to help prevent timing attacks
            //let _ = encryption::hash(secret, &params.password);
            //return Err(ApiError::Unauthorized);
        //},
    //};

    //if !encryption::verify(secret, &user.password, &params.password)? {
        //return Err(ApiError::Unauthorized);
    //}

    //Ok(req.session()
        //.set("user_id", &user.id)
        //.map(|_| Response::Ok().finish())?)
//}

//pub fn delete(req: Request) -> Response {
    //req.session().clear();
    //Response::Ok().finish()
//}
