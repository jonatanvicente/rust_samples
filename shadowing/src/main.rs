fn main() {
    let x = 5;
    //declaramos con mismo nombre (shadowing). Dará error si no usamos let.
    //al usar let, realmente es una nueva variable, no modifica la anterior
    let x = x + 1;//x = 6

    //creamos ámbito interno con los curly brackets
    {
        //shadows de nuevo sobre x
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");//x = 12
    }
    //x vuelve a ser 6
    println!("The value of x is: {x}");
}