use actix_web::dev::RequestHead;

use crate::{
    extensions::Assigns,
    store::authors::AuthorWithoutPassword,
};

pub fn auth_guard(head: &RequestHead) -> bool {
    let author: Option<AuthorWithoutPassword> = head
        .extensions()
        .get::<Assigns>()
        .map(|assn| assn.author.clone())
        .flatten();

    author.is_some()
}
