#[derive(Debug)] //para poder imprimir el struct
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    //println!("rect1 is {}", rect1);//ERROR, no se puede imprimir un struct
    println!("rect1 is {:?}", rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

//OJO al parámetro: mejor recibir una referencia que tomar el ownership de él
fn area(rectangle: &Rectangle) -> u32 {
    //OJO aquí: tomar los parámetros de la referencia no hace move de estos valores
    //por eso a menudo hay borrows (préstamos, y así no ownership) de referencias
    rectangle.width * rectangle.height
}