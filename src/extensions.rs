use crate::models::AuthorWithoutPassword;

#[derive(Debug)]
pub struct Assigns {
    pub author: Option<AuthorWithoutPassword>,
}
