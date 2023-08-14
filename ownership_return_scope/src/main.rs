/*

IMPORTANTE:
The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it.
When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless
ownership of the data has been moved to another variable.
 */

fn main() {
    //s1 obtiene valor de variable creada en otra función
    let s1 = gives_ownership();         // gives_ownership moves its return value into s1
    let s2 = String::from("hello");   // s2 comes into scope
    let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back, which also moves its return value into s3
    //println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);//error al imprimir s2: fue movido
    println!("s1: {}, s3: {}", s1, s3);//s2 está en s3
}
// Here, s3 goes out of scope and is dropped.
// s2 was moved, so nothing happens.
// s1 goes out of scope and is dropped.


// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    let some_string = String::from("yours"); // some_string comes into scope
    some_string
    // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
// a_string comes into scope
    a_string  // a_string is returned and moves out to the calling function
}