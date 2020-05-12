use futures::FutureExt;
use sqlx::PgPool;
use std::env;

#[derive(Debug)]
pub struct State {
    pub pool: PgPool,
    pub secret_key: String,
    pub upload_path: String,
}

impl State {
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
