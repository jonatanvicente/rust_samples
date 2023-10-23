struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T { //returns a reference to the data in the field x
        &self.x
    }
}

impl Point<f32> { //only implemented for a particular type
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p = Point { x: 5.0, y: 10.0 };
    println!("p.distance_from_origin = {}", p.distance_from_origin());
}