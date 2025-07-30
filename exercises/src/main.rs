mod fibonacci;
mod nested_arrays;
mod collatz_sequence;

pub use crate::fibonacci::fib;

fn main() {
    //fibonacci example
    let n = 5;
    println!("Fib({n}) = {}", fib(n));

    //Collatz sequence example
    println!("Length: {}", crate::collatz_sequence::collatz_length(11)); // should be 15

    // nested arrays example
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    println!("Transposed = {:?}", crate::nested_arrays::transpose(matrix));
}



