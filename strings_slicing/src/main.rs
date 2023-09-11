fn main() {

    let hello = "Здравствуйте"; //cada character son 2 bytes
    let s = &hello[0..4]; //1os 4 bytes
    println!("S: {}", s);
}
