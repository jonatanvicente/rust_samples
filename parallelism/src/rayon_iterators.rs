// Parallel iterator example using rayon
use rayon::prelude::*;

//Data parallelism example using rayon
pub fn main() {
    let words = vec!["rust", "is", "fast", "and", "safe"];
    let uppercased: Vec<String> = words.par_iter()
        .map(|w| w.to_uppercase())
        .collect();
    println!("Uppercased words: {:?}", uppercased);
}

