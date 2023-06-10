
use std::env;
use crate::api::crypto;

pub fn cli() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "crypto" {
        crypto()
    } else {
        println!("Invalid command. Usage: cargo run crypto");
    }
}