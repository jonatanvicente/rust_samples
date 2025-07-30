/*
let array = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
Use an array such as the above to write a function transpose which will transpose a matrix (turn rows into columns):
 */

pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}


