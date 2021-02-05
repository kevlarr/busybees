//! A simple command-line tool to accept a password, encrypt it,
//! and print to std out.

use busybees::encryption;
use std::env;

fn main() {
    dotenv::dotenv().ok();

    let secret = env::var("HASH_SECRET").expect("No hash secret specified");
    let args: Vec<String> = env::args().collect();

    match encryption::hash(&secret, &args[1]) {
        Ok(hash) => println!("{}", hash),
        Err(e) => eprintln!("{}", e),
    }
}
