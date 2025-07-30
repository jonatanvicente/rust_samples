mod fibonacci;
mod nested_arrays;
mod collatz_sequence;
mod geometry;

pub use crate::fibonacci::fib;

fn main() {

    //fibonacci example
    let n = 5;
    println!("Fib({n}) = {}", fib(n));

    //Collatz sequence example
    println!("Length: {}", collatz_sequence::collatz_length(11)); // should be 15

    // nested arrays example
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    println!("Transposed = {:?}", nested_arrays::transpose(matrix));

    // Geometry - vector magnitude and normalization
    println!("Magnitude of a unit vector: {}", geometry::magnitude(&[0.0, 1.0, 0.0]));
    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", geometry::magnitude(&v));
    geometry::normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", crate::geometry::magnitude(&v));

}



