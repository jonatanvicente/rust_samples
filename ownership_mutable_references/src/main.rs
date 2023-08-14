
fn main() {
    let mut s = String::from("Hello");
    change(&mut s);
    println!("{}", s);

    //two_mutable_references_error();//ERROR
    //combining_mutable_and_immutable_references();//ERROR
    two_mutable_references();
    reference_scope();

    //Rust evita punteros colgantes (referencias a ningún sitio)
    //let reference_to_nothing = dangle();
    let reference_ok = no_dangle();
}

fn change(some_string: &mut String) {
    some_string.push_str(", World");//mutará el valor prestado (borrowed)
}

//ERROR
//RESTRICCION: no se pueden tener dos referencias mutables a un mismo valor en un mismo ámbito

/*
The benefit of having this restriction is that Rust can prevent data races at compile time.
A data race is similar to a race condition and happens when these three behaviors occur:
    - Two or more pointers access the same data at the same time.
    - At least one of the pointers is being used to write to the data.
    - There’s no mechanism being used to synchronize access to the data.

 */
/*
fn two_mutable_references_error() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;//no se puede hacer borrow (prestar la referencia mutable) más de una vez

    println!("{}, {}", r1, r2);

}
*/

//solución: crear un nuevo ámbito
fn two_mutable_references () {
    let mut s = String::from("hello pp");
    {
        let r1 = &mut s;
        println!("{}", r1);
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
    println!("{}", r2);
}

/*
fn combining_mutable_and_immutable_references() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);

}
*/

/*
Note that a reference’s scope starts from where it is introduced and continues through the last time
 that reference is used. For instance, this code will compile because the last usage of the immutable
  references, the println!, occurs before the mutable reference is introduced:
 */
fn reference_scope() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

}
//Rust evita punteros colgantes
//ERROR EN COMPILACIÓN: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
/*fn dangle() -> &String {
    let s = String::from("hello");
    &s // we return a reference to the String, s
}
// Here, s goes out of scope, and is dropped. Its memory goes away.
//No podemos intentar pasar una referencia a ella, se está apuntando a un String inválido
  // Danger!
  */


fn no_dangle() -> String {
    let s = String::from("hello");
    s
}//Ownership is moved out, and nothing is deallocated.