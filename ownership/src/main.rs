//Ejemplos basados en string literal y String type
fn main() {
    scope_string_literal();
    string_type();
    scope_string_type();
}

//ejemplo con string literal
fn scope_string_literal() {

    //creamos ámbito
    {                   // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward
        println!("{}", s); //OJO: si comentamos esta línea no compila: variable sin utilizar
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid

    //string literals muy eficientes: van directos al ejecutable. Su tamaño y contenido son fijos y conocidos por el compiler
}


fn string_type() {

    //String type puede ser mutado (podemos ponerle caracteres, quitarle, etc), pero string no
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`

    //La memoria será solicitada del allocator (heap) at runtime (con la llamada String::from)
    /*
        Necesitaremos retornar o liberar la memoria al allocator cuando ya hemos utilizado la variable (no tenemos GC):
        - Si nos olvidamos, desperdiciaremos memoria
        - Si lo hacemos antes de tiempo, tendremos una variable inválida
        - Si lo hacemos dos veces, tendremos un error. Deberemos emparejar exactamente un allocate con un free


     */
}


fn scope_string_type() {

    {
        let s = String::from("hello");
        println!("{}", s);

    }
    /*
        IMPORTANT:
        When a variable goes out of scope, Rust calls a special function for us.
        This function is called drop, and it’s where the author of String can put the code to return the memory.
        Rust calls drop automatically at the closing curly bracket.
     */

}
