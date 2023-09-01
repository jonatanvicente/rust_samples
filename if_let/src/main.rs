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

fn match_sample(coin: Coin) {
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}


fn if_let_sample(coin: Coin) {
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
        println!("Count now is {count}");
    }
}



fn main() {
    match_sample(Coin::Quarter(UsState::Alaska));
    if_let_sample(Coin::Penny);
    if_let_sample(Coin::Nickel);
    if_let_sample(Coin::Dime);
    if_let_sample(Coin::Quarter(UsState::Alaska));
    if_let_sample(Coin::Quarter(UsState::Alabama));
}
