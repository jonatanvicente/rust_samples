fn main() {
    /*
    let x = 5;
    println!("The value of x is: {x}");
    //ERROR: variable inmutable por defecto
    x = 6;
    println!("The value of x is: {x}");
     */
    let mut x = 5;//mutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //boolean
    let t = true;
    let f: bool = false; // with explicit type annotation
    let c = 'z'; //String uses double quotes
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}