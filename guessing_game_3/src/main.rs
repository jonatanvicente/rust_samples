use std::io;
use std::cmp::Ordering; //The Ordering type is another enum and has the variants Less, Greater, and Equal.
// These are the three outcomes that are possible when you compare two values.
use rand::Rng;


fn main() {

    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");
    println!("Please input your guess.");

    //OJO variable reutilizada más adelante (es mut)
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)//pasamos como parámetro la variable mut creada arriba para que vuelque valor introducido por user
        //& indica que el valor es una referencia. Nos permitirá acceder a esa pieza de datos sin necesidad de copiarla en memoria varias veces
        //referencias son inmutables por default
        //readline añade \n al final de la entrada. trim() más abajo elimina el \n
        .expect("Failed to read line");

    //shadowing: reutilización de la variable creada anteriormente
    //vinculamos la variable anterior
    //guess.trim hace referencia a la variable original
    //parse() convierte el string en un tipo numérico. Necesario tipar guess (u32)
    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");

    //en la comparación, Rust infiere que ambas variables son u32
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}