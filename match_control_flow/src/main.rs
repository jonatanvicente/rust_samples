#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    //match efectúa la comparación en orden. Si el patrón coincide, se ejecuta el código asociado. Si no, se pasa al siguiente patrón.
    match coin { //evaluación de cualquier tipo de dato
        Coin::Penny => 1,
        Coin::Nickel => 5, //pattern => codigo a ejecutar
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

//matching con Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn catch_all_values(num:i32) {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        other => move_player(other), //captura de valores restantes
        //_ => println!("anything... {num}"), //captura de valores restantes. Evitamos warning de variable no usada
    }
}

fn move_player(num_spaces: u8) {
    println!("move player -> {num_spaces}")
}


fn main() {
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alaska));
    value_in_cents(Coin::Quarter(UsState::Alabama));
    //
    let five = Some(5);
    let _six = plus_one(five); //al poner underscore evitamos warning de variable no usada
    let _none = plus_one(None);

    //
    catch_all_values(5);
}
