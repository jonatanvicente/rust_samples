#![allow(warnings)]
mod fibonacci;
mod nested_arrays;
mod collatz_sequence;
mod geometry;
mod elevator_events;
mod expression_evaluation;
mod logger_trait;
mod counter;
mod generic_min;
mod log_filter;
mod ROT13;
mod health_statistics;
mod protobuf;
mod iterator_chaining;
mod pointers;
mod ownership;
mod luhn_algorithm;
mod result_rewriting;
mod FFI_wrapper;
mod reverse;
mod mut_references;

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
    log_filter::go();
    pointers::go();

}



