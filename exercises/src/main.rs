#![allow(warnings)]
mod fibonacci;
mod nested_arrays;
mod collatz_sequence;
mod geometry;
mod elevator_events;
mod expression_evaluation;
mod logger_trait;
mod counter;

fn main() {

    //fibonacci example
    fibonacci::go();
    //Collatz sequence example
    collatz_sequence::go();
    // nested arrays example
    nested_arrays::go();
    // Geometry - vector magnitude and normalization
    geometry::go();
    // elevator events
    elevator_events::go();
    // Logger trait
    logger_trait::go();
    //counter
    counter::go();

}



