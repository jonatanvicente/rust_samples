fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s se movió a la función
    // s ya no es válida aquí
    //println!("{}", s); //error de compilación, se ha hecho move



    let x = 5;  // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it's okay to still

    println!("****** {}", x);//es válido, no se ha hecho move (integers están en stack)
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.