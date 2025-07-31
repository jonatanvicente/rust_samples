/*
let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
Use an array such as the above to write a function transpose which will transpose a matrix (turn rows into columns):
 */
use crate::nested_arrays;

pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

pub fn go() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    println!("Transposed = {:?}", transpose(matrix));
}


