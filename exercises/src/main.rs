mod fibonacci;
pub use crate::fibonacci::fib;

fn main() {
    let n = 5;
    println!("fib({n}) = {}", fib(n));
}

