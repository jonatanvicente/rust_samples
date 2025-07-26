fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        /*if item > largest { //error: binary operation `>` cannot be applied to type `&T`
            largest = item;
        }*/
    }

    largest
}

//respecto a generic_functions_1, añadimos genérico, pero dará error
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}