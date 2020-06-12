//! Types for extending the request instances.
use busybees::store::authors::AuthorWithoutPassword;

/// A container type that serves as a target for middlewares that
/// need to add data to individual requests.
#[derive(Debug)]
pub struct Assigns {
    pub author: Option<AuthorWithoutPassword>,
}
