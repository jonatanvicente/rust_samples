use std::io;//para obtener info de teclado del user, módulo std:prelude

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); //mut = muttable. Variables por defecto inmutables en Rust

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}