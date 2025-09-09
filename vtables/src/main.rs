use vtables::{go, go_again};

trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

fn make_speak(animal: &dyn Animal) {
    animal.speak(); // Uses vtable for dynamic dispatch
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    make_speak(&dog);
    make_speak(&cat);

    //~~~~~~~~~~~~

    go();
    go_again();
}