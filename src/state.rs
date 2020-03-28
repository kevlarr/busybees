use futures::FutureExt;
use sqlx::PgPool;
use std::{cell::RefCell, env, rc::Rc};

pub struct State {
    pub pool: Rc<RefCell<PgPool>>,
    pub secret_key: String,
}

impl State {
    pub fn new() -> Self {
        let secret_key = env::var("HASH_SECRET").expect("HASH_SECRET not set");
        let url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let pool = PgPool::new(&url)
            .now_or_never()
            .unwrap() // futures Option
            .unwrap(); // sqlx Result

        State {
            pool: Rc::new(RefCell::new(pool)),
            secret_key,
        }
    }
}
