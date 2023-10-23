struct Point<T> {
    x: T,
    y: T,
}

struct AnotherPoint<T, U> { //corregimos para añadir tipos diferentes
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    //let other = Point { x: 5, y: 4.0 };//error

    let both_integer = AnotherPoint { x: 5, y: 10 };
    let both_float = AnotherPoint { x: 1.0, y: 4.0 };
    let integer_and_float = AnotherPoint { x: 5, y: 4.0 };
}