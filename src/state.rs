use futures::FutureExt;
use sqlx::PgPool;
use std::{cell::RefCell, env, rc::Rc};

pub struct State {
    pub pool: Rc<RefCell<PgPool>>,
}

impl State {
    pub fn new() -> Self {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

        let pool = PgPool::new(&url)
            .now_or_never()
            .unwrap() // futures Option
            .unwrap(); // sqlx Result

        State {
            pool: Rc::new(RefCell::new(pool)),
        }
    }
}
