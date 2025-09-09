trait Shape {
    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 { 3.14 * self.radius * self.radius }
}

struct Square { side: f64 }
impl Shape for Square {
    fn area(&self) -> f64 { self.side * self.side }
}

pub fn go() {
    /*
        dyn keyword specifies that we are using a trait. This allows storing different
        types here (Circle and Square) that implement the Shape trait.

        At runtime, Rust uses a vtable (virtual method table) to call the correct area method
        for each object. dyn Shape tells Rust to use dynamic dispatch and calls the correct
        implementation.
        This enables polymorphism.
     */

    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Square { side: 3.0 }),
    ];
    for shape in shapes {
        println!("Area: {}", shape.area()); // vtable used here
    }
}

//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

trait Writer {
    fn write(&self, msg: &str);
}

struct ConsoleWriter;
impl Writer for ConsoleWriter {
    fn write(&self, msg: &str) {
        println!("{}", msg);
    }
}

/*
     Creates a trait.
     The exact type is not known at compile time (only that it implements Writer).
 */
fn get_writer() -> Box<dyn Writer> {
    Box::new(ConsoleWriter)
}

pub fn go_again() {
    let writer = get_writer(); //Rust uses vtable to dynamically dispatch the call to the correct write method
    //enables polymorphism at runtime
    writer.write("Hello, vtable!"); // vtable used for method call
}