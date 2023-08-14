//Slices let you reference a contiguous sequence of elements in a collection rather than the whole
// collection. A slice is a kind of reference, so it does not have ownership.

fn main() {

    let s = String::from("hello world");
//A string slice is a reference to part of a String
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("hello {}", hello);
    println!("world {}", world);

    let s = String::from("hello");
    let slice = &s[0..2];
    let slice = &s[..2];//empieza desde el primer índice (0)
    println!("slice {}", slice);

    let s = String::from("hello");
    let len = s.len();
    let slice = &s[3..len];//hasta el final
    let slice = &s[3..];
    println!("slice {}", slice);

    //tomando todos los valores...
    let s = String::from("hello");
    let len = s.len();
    let slice = &s[0..len];
    let slice = &s[..];

    println!("slice {}", slice);
    let s = String::from("Hola pp");
    first_word(&s);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

