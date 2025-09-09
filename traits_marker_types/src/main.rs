#![allow(warnings)]

struct Grounded; //to represent each state of the rocket
struct Launched;
struct Color; //dummy struct
struct Kilograms; //dummy struct
struct Rocket<Stage = Grounded> { //You can construct a Rocket only in the Grounded state
    stage: std::marker::PhantomData<Stage>,//guarantees that is eliminated at compile time
}

impl Default for Rocket<Grounded> {
    fn default() -> Self {
        todo!()
    }
}

impl Rocket<Grounded> {
    pub fn launch(self) -> Rocket<Launched> { Rocket { stage: std::marker::PhantomData } }
}

//Only when the rocket is been launched can you control its velocity
impl Rocket<Launched> {
    pub fn accelerate(&mut self) { }
    pub fn decelerate(&mut self) { }
}
//Generic implementation block: some things you can do it no matter what state it is in
// Is not possible for the user call a method at the wrong time
impl<Stage> Rocket<Stage> {
    pub fn color(&self) -> Color { Color }
    pub fn weight(&self) -> Kilograms { Kilograms}
}
//#[must _use] //Added to any type, trait, or function, and the compiler will issue a warning if the user’s code receives an element
// of that type or trait, or calls that function, and does not explicitly handle it.
fn main() {
    println!("Hello, world!");
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
fn is_normal<T: Sized + Send + Sync + Unpin>() {}

//Testing that a type implements a set of traits
#[test]
fn normal_types() {
    is_normal::<Rocket>();
}
