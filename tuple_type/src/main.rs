fn main() {
    //OJO diferentes tipos, agrupados
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    //Declaración para poder desestructurar
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    println!("The value of z is: {z}");


    //otra posibilidad
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}