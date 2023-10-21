use std::fs::File;

fn main() {

    //providing good error messages can convey your intent and make tracking down the source of a panic easier.
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");//error showed if file not found
}

/*
In production-quality code, most Rustaceans choose expect rather than unwrap and give more context
 about why the operation is expected to always succeed. That way, if your assumptions are ever
 proven wrong, you have more information to use in debugging.
 */