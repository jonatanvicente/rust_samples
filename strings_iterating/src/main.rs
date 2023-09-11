fn main() {

    //Remember that valid Unicode scalar values may be made up of more than 1 byte.
    //See doc at https://doc.rust-lang.org/book/ch08-02-strings.html#bytes-and-scalar-values-and-grapheme-clusters-oh-my
    let hello = "Здравствуйте"; //2 bytes cada char
    let s = &hello[0..4];
    println!("S: {}", s);

    for c in "Зд".chars() {
        println!("{c}");
    }

    for b in "Зд".bytes() {
        println!("{b}");
    }

}
