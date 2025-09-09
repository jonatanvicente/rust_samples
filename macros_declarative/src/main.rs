#![allow(unused_variables)]
use std::any::{type_name};


/*
  //a declarative macro (generally) cannot affect variables that aren’t explicitly passed to it

(macro_rules!)macro_rules! /* macro name */ {
        (/* 1st matcher */) => { /* 1st transcriber */ };
        (/* 2nd matcher */) => { /* 2nd transcriber */ };
        }
 */
#[macro_export]
macro_rules! my_macro {  //declarative macro: use pattern matching to generate code; procedural macros require separate crate with special attributes
    () => {
        println!("Hello from macro!");
    };
}

macro_rules! let_foo {
    ($x:expr) => {
        let foo = $x;
    }
}

// You can choose to share identifiers between a macro and its caller if you want the macro to affect a specific variable in the caller’s scope
macro_rules! please_set {
    ($i:ident, $x:expr) => {
        $i = $x;
    }
}




fn main() {

    my_macro!();
    // ~~~~~~~~~~~~~~~~~
    let foo = 1;
    // expands to let foo = 2;
    let_foo!(2);
    assert_eq!(foo, 1); //does not change

    // ~~~~~~~~~~~~~~~~~
    // share identifiers
    let mut x = 1;
    please_set!(x, x + 1);
    assert_eq!(x, 2);
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



