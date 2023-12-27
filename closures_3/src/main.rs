use closures_3::{closure_definition, closure_mut, closure_move, closure_fnmut, closure_fnonce, closure_fnmut_2};

fn main() {
    
    closure_definition();
    println!("~~~~~~~~~~~~~~~~~~~~");
    closure_mut();
    println!("~~~~~~~~~~~~~~~~~~~~");
    closure_move();
    println!("~~~~~~~~~~~~~~~~~~~~");
    closure_fnmut();
    println!("~~~~~~~~~~~~~~~~~~~~");
    closure_fnmut_2();
    closure_fnonce();
    
}


