use std::io;
fn main() {

    let _a: [i32; 5] = [1, 2, 3, 4, 5]; //  _[varName] -> var never used

    //You can also initialize an array to contain the same value for each element by specifying the initial value,
    // followed by a semicolon, and then the length of the array in square brackets
    //This is the same as writing let a = [3, 3, 3, 3, 3]; but in a more concise way.
    let a = [3; 5];

    println!("The value of a is: {:?}", a);
    error_accessing_array();

}


fn error_accessing_array() {

    //si intentamos trazar un elemento fuera de array:
    //thread 'main' panicked at 'index out of bounds: the len is 5 but the index is [numeroPuesto]'

        let a = [1, 2, 3, 4, 5];
        println!("Please enter an array index.");

        let mut index = String::new();

        io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");

        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");

        let element = a[index];

        println!("The value of the element at index {index} is: {element}");
}