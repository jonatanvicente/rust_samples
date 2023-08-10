//importar librería estándar de Rust
use std::io;//para obtener info de teclado del user, módulo std:prelude
use rand::Rng; //para random


//La idea es un juego en el cual el usuario introduce un número, para adivinar un random de la máquina
fn main() {
    println!("Guess the number!");

    //usamos un generador de números aleatorios. Es local al thread de ejecución
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //cada crate (caja) tiene doc con instrucciones para usarla
    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    //::new es función asociada a String. Retorna un nuevo String vacío
    //función asociada = función implementada en un tipo
    let mut guess = String::new(); //mut = muttable. Variables por defecto inmutables en Rust

//podríamos usar std::io::stdin() sin la importación de arriba
    io::stdin()
        .read_line(&mut guess)//pasamos como parámetro la variable mut creada arriba para que vuelque valor introducido por user
        //& indica que el valor es una referencia. Nos permitirá acceder a esa pieza de datos sin necesidad de copiarla en memoria varias veces
        //referencias son inmutables por default
        .expect("Failed to read line"); //si es ok en lugar de err, retornará el valor para usarlo (los bytes introducidos).
    //Si es err, se ejecuta el mensaje de error

    println!("You guessed: {guess}"); //muestra la variable introducida
}