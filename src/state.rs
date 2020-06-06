//! Server application state
use futures::FutureExt;
use sqlx::PgPool;
use std::env;

/// Container for singleton objects needed across different modules.
#[derive(Debug)]
pub struct State {
    pub pool: PgPool,
    pub secret_key: String,
    pub upload_path: String,
}

impl State {
    /// Returns a new `State` that stores the provided system path to uploaded files
    /// and that loads other details from environment.
    #[deprecated(note = "State should load upload path itself")]
    pub fn new(upload_path: String) -> Self {
        let secret_key = env::var("HASH_SECRET").expect("HASH_SECRET not set");
        let url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let pool = PgPool::new(&url)
            .now_or_never()
            .unwrap() // futures Option
            .unwrap(); // sqlx Result

        State {
            pool,
            secret_key,
            upload_path,
        }
    }
}
