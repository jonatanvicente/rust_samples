#![allow(warnings)]
mod fibonacci;
mod nested_arrays;
mod collatz_sequence;
mod geometry;
mod elevator_events;

fn main() {

    //fibonacci example
    let n = 5;
    println!("Fib({n}) = {}", fibonacci::fib(n));

    //Collatz sequence example
    println!("Length: {}", collatz_sequence::collatz_length(11)); // should be 15

    // nested arrays example
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];
    println!("Transposed = {:?}", nested_arrays::transpose(matrix));

    // Geometry - vector magnitude and normalization
    println!("Magnitude of a unit vector: {}", geometry::magnitude(&[0.0, 1.0, 0.0]));
    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", geometry::magnitude(&v));
    geometry::normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", crate::geometry::magnitude(&v));


    // elevator events
    println!(
        "A ground floor passenger has pressed the up button: {:?}", //necessary debug enabled
        elevator_events::lobby_call_button_pressed(0, elevator_events::Direction::Up)
    );
    println!("The car has arrived on the ground floor: {:?}", elevator_events::car_arrived(0));
    println!("The car door opened: {:?}", elevator_events::car_door_opened());
    println!(
        "A passenger has pressed the 3rd floor button: {:?}",
        elevator_events::car_floor_button_pressed(3)
    );
    println!("The car door closed: {:?}", elevator_events::car_door_closed());
    println!("The car has arrived on the 3rd floor: {:?}", elevator_events::car_arrived(3));


}



