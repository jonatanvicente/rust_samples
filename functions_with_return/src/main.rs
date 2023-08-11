fn five() -> i32 {
    5  //valor de retorno (OJO SIN ;)
}

fn main() {
    let x = five(); //usamos el valor de retorno de una función para inicializar una variable
    //esto sería equivalente a let x = 5;

    let y = plus_one(5);
    println!("The value of x is: {x}");
    println!("The value of y is: {y}");
}

fn plus_one(x: i32) -> i32 {
    x + 1//si ponemos ; dará error
}