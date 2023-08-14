//Ejemplos basados en string literal y String type
fn main() {
    scope_string_literal();
    string_type();
    scope_string_type();
    move_variables ();
    clone_variables();
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



fn move_variables() {
    let s1 = String::from("hello");
    let s2 = s1;//s1 was moved to s2

    //println!("{}, world!", s1); //NOT WORKING: Rust impide que s1 sea usada después de mover su valor a s2. ESTO SE DENOMINA MOVE
    /*
      OJO MOVE NO OCURRE CON INTEGERS o con tipos cuya size son conocidas en tiempo de compilación (ambos van a stack, ninguno a heap).
      Por eso, este código sería válido:
            let x = 5;
            let y = x;
            println!("x = {}, y = {}", x, y);
     */

    println!("{}, world!", s2);

    /*
        PROCESO EN MEMORIA:
        - s1 en stack: es un puntero que contiene los siguientes campos:
            ptr: puntero al contenido del string (que se aloja en heap)
            len: longitud del string
            capacity: total amount of memory in bytes
         - s1 en heap: contiene el contenido del string, con una celda para cada letra y un index empezando en 0 que lo determina
         - s2: el símbolo igual copia el puntero PERO NO LA DATA RESIDENTE EN HEAP.
         - Si quieremos copiar todo (puntero y data, llamado DEEP COPY), debemos usar el método clone() (ver más abajo)

         LIMPIEZA DE VARIABLES:
         - Cuando s2 y s1 salgan del scope, Rust llamará a DROP para limpiar el heap de la variable.
         - Esto generaría el double free error: ambas dejarán libre la misma memoria heap. Esto genera vulnerabilidades de seguridad.
         - PARA EVITARLO, Rust ya no considera válida s1 después de asignar s2.
         - Rust nunca creará copias 'deep' de nuestra data. Cualquier copia que haga no incidirá en rendimiento.

        USO DE COPY:
        - Rust no nos dejará usar COPY de un tipo que implemente DROP.
        - No habrá MOVE si el tipo implementa COPY.
        Here are some of the types that implement Copy:
            - All the integer types, such as u32.
            - The Boolean type, bool, with values true and false.
            - All the floating-point types, such as f64.
            - The character type, char.
            - Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.

     */

}

fn clone_variables () {

    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

}