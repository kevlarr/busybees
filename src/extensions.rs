use crate::store::authors::AuthorWithoutPassword;

#[derive(Debug)]
pub struct Assigns {
    pub author: Option<AuthorWithoutPassword>,
}
