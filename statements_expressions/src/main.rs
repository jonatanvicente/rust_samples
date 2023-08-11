
//Statement (declaración). Las funciones lo son
//Statements do not return values (IMPORTANTE).
fn main() {

    //statement (variable)
    let y = 6;

    /*
       OJO aquí:
       let x = (let y = 6); //error. Al no retornar nada, no se puede vincular a otra variable
       let x = y = 6; //error, ídem

       This is different from what happens in other languages, such as C and Ruby,
       where the assignment returns the value of the assignment.
       In those languages, you can write x = y = 6 and have both x and y have the value 6; that is not the case in Rust.
     */

    println!("The value of y is: {y}");
    expression();
}

//OJO expressions son diferentes: evalúan y retornan un valor
fn expression() {

    //creamos ámbito
    let y = {
        let x = 3;
        x + 1 //OJO expressions don't include ending semicolons.
        // OJO: If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.
    };
    println!("The value of y is: {y}");
}