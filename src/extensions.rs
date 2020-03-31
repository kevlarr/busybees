use crate::models::AuthorWithoutPassword;

pub struct Assigns {
    pub user: Option<AuthorWithoutPassword>,
}
