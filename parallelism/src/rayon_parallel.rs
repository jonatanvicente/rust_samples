// Data parallelism example using rayon
use rayon::prelude::*;

// Parallel iterators with rayon
pub fn main() {
    let numbers: Vec<u32> = (1..=10).collect();
    let squares: Vec<u32> = numbers.par_iter().map(|n| n * n).collect();
    println!("Squares: {:?}", squares);
}

