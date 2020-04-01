use crate::models::AuthorWithoutPassword;

#[derive(Debug)]
pub struct Assigns {
    pub user: Option<AuthorWithoutPassword>,
}
