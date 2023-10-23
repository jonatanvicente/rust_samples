pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);//llamada a panic
        }
        Guess { value } //creación de variable sólo con valor entre 1 y 100
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


fn main() {
    let _guess = Guess::new(101);//provocamos error
}

