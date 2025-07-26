pub struct Guess {
    pub value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

fn main() {
	  let guess = Guess::new(50);
	  print!("Guess value must be between 1 and 100, got {}.", guess.value);
}