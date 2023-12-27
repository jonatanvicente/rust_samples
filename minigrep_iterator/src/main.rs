use std::{env, process};
use minigrep_iterator::Config;

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
    
    //  env::args() -> retorna un iterator
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    
    
    if let Err(e) = minigrep_iterator::run(config) {
        process::exit(1);
    }
    
}

