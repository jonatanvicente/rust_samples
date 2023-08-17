#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
    Es necesario definir los métodos en el contexto de un struct.
    Todo aquí será asociado a un Rectangle.
    Será necesario poner siempre el parámetro self en la firma y en cualquier lugar del body
 */
impl Rectangle {
    //The &self is actually short for self: &Self. Within an impl block, the type Self is an alias
    // for the type that the impl block is for.
    //no queremos el ownership, solo una referencia. Sólo queremos leer, no escribir
    //If we wanted to change the instance that we’ve called the method on as part of what the method does,
    // we’d use &mut self as the first parameter.
    /*
    ¿Por qué usar métodos en lugar de funciones?
    The main reason for using methods instead of functions, in addition to providing method syntax
    and not having to repeat the type of self in every method’s signature, is for organization.
    We’ve put all the things we can do with an instance of a type in one impl block rather than
    making future users of our code search for capabilities of Rectangle in various places in the
    library we provide.
     */
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() //invocación del método
    );
}