use std::fs::File;


//shortcut for panic on error
fn main() {

    //unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.
    let greeting_file = File::open("hello.txt").unwrap();//file not exists
}