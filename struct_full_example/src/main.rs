
//Ejemplo de un programa que calcula el área de un rectángulo

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

//asuntos a revisar: no está claro que ambos parámetros estén relacionados (ancho y alto)
fn area(width: u32, height: u32) -> u32 {
    width * height
}