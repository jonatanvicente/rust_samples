use tests_panic::Guess;

fn main() {
    let guess = Guess::new(50);
	  print!("Guess value must be between 1 and 100, got {}.", guess.value);
}