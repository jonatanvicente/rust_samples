#![allow(unused_variables)]
use std::any::{type_name};

#[macro_export]
macro_rules! my_macro {  //declarative macro: use pattern matching to generate code; procedural macros require separate crate with special attributes
    () => {
        println!("Hello from macro!");
    };
}


fn main() {
    my_macro!();
}


fn test_inner<T>(init: T, frobnify: bool) -> bool{
    type_name::<T>() == "u8"
}

#[test]
fn test_1u8_frobnified() {
    test_inner(1u8, true);
}


#[test]
fn test_1i128_not_frobnified() {
    test_inner(1i128, false);
}



