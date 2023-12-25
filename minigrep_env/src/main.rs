use std::{env, process};
use minigrep_env::Config;

#[allow(unused_variables)]

/*
    cargo run -- frog poem.txt
    cargo run -- body poem.txt
    cargo run -- monomorphization poem.txt
    cargo run -- to poem.txt
    IGNORE_CASE=1 cargo run -- to poem.txt (fijamos env var para esa ejecución)
    cargo test
 */

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        process::exit(1);
    });
    
    
    
    if let Err(e) = minigrep_env::run(config) {
        process::exit(1);
    }
    
}

