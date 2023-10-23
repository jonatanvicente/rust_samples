//selección del número más alto de una lista
fn main() {

    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = &number_list[0]; //almacenamiento temporal del más alto valor hasta el momento
    //inicializamos con primer valor del array

    for number in &number_list { //iteración
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}