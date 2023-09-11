fn main() {

    //Ejemplo 1
    //podríamos crear string vacío y añadir data, pero tenemos data inicial en una variable
    let data = "initial contents";

    //usamos to_string method
    let s = data.to_string();
    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    /*
    También valdría:
        let s = String::from("initial contents");
     */

    //Ejemplo 2
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    println!("{}", hello);

    //append to string
    let mut s = String::from("foo");
    s.push_str("bar");

    //
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");


    //
    let mut s = String::from("lo");
    s.push('l');//un sólo char

    //
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = s1 + "-" + &s2 + "-" + &s3;

    //
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}"); //works like println

    //ERROR: Rust does not support indexing strings
/*    let s1 = String::from("hello");
    let h = s1[0];*/


/*
OJO:
    Three relevant ways to look at strings from Rust’s perspective:
    as bytes, scalar values, and grapheme clusters (the closest thing to what we would call letters).
 */


}
