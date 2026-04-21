//Goal: Fix the ownership error.
// In this snippet, we try to use a variable after its ownership has been moved into a function.

/*
fn run() {
    let s1 = String::from("Rust");

    print_length(s1);

    // ERROR: value borrowed here after move
    println!("The string was: {}", s1);
}

fn print_length(s: String) {
    println!("Length: {}", s.len());
}
 */

// Solution
pub fn run() {
    let s1 = String::from("Rust");
    print_length(s1.as_str());
    println!("The string was: {}", s1);
}

fn print_length(s: &str) {
    println!("Length: {}", s.len());
}