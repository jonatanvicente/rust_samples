/*
    Inheritance and Polymorphism:
    -  Vehicle is a trait with a method drive.
    - Bicycle, Car, and Plane are structs implementing Vehicle.
 */


trait Vehicle {
    fn drive(&self);
}

struct Bicycle;
struct Car;
struct Plane;

impl Vehicle for Bicycle {
    fn drive(&self) {
        println!("Pedaling the bicycle!");
    }
}

impl Vehicle for Car {
    fn drive(&self) {
        println!("Driving the car!");
    }
}

impl Vehicle for Plane {
    fn drive(&self) {
        println!("Flying the plane!");
    }
}

pub fn go() {
    let vehicles: Vec<Box<dyn Vehicle>> = vec![
        Box::new(Bicycle),
        Box::new(Car),
        Box::new(Plane),
    ];

    for v in vehicles {
        v.drive();
    }
}