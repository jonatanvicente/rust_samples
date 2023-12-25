use std::{env, process};
use minigrep_errs::Config;

#[allow(unused_variables)]

/*
    cargo run -- frog poem.txt
    cargo run -- body poem.txt
    cargo run -- monomorphization poem.txt
    cargo run -- to poem.txt
    IGNORE_CASE=1 cargo run -- to poem.txt (fijamos env var para esa ejecución)
    cargo test
    
    OJO aquí: al ejecutar
    cargo run -- to poem.txt > output.txt redireccionamos la salida standard a file (terminación del programa exitosa)
    standard error para error output (eprintln!)

 */

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}"); //standard error
        process::exit(1);
    });
    
    if let Err(e) = minigrep_errs::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
    
}

