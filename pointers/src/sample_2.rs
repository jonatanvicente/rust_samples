//Goal: Understand the rules of mutable borrowing.
// Rust prevents data races by allowing only one mutable reference at a time.

fn run() {
    let mut balance = 100;

    let deposit = &mut balance;
    let withdraw = &mut balance; // ERROR: second mutable borrow occurs here

    *deposit += 50;
    *withdraw -= 20;

    println!("Final balance: {}", balance);
}