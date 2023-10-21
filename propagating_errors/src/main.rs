use std::fs;
use std::fs::File;
use std::io::{self, Read};


//see https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#propagating-errors
fn main() {

    let _var = read_username_from_file();
    let _var_again = read_username_from_file_again();
    let _var_again_simple = read_username_from_file_simple();
    let _var_again_simple_2 = read_username_from_file_simple_2();

}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };


    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

//using ? operator instead of above solution (match)
fn read_username_from_file_again() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_simple() -> Result<String, io::Error> {

    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_simple_2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}


