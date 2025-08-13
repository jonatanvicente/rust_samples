pub fn go() {
    let x1 = 42; //implements Copy trait
    let y1 = Box::new(84);
    { // starts a new scope
        let z = (x1, y1);
        // z goes out of scope, and is dropped;
        // it in turn drops the values from x1 and y1
    }
    // x1's value is Copy, so it was not moved into z
    let x2 = x1;
    println!("{:}", x2);
    //println!("{:}", z); //err: out of scope
    // y1's value is not Copy, so it was moved into z
    // let y2 = y1;

}

fn cache(input: &i32, sum: &mut i32) {
    *sum = *input + *input;
    assert_eq!(*sum, 2 * *input);
}

/*
    We cannot return a reference (pointer) to a local variable because it will be dropped
    when the function ends, leading to a dangling reference.
    Instead, we can return references to data that lives longer than the function (e.g. passed
    in as arguments) or owned data (Box, String or Vec), which transfers ownership to the caller.

    fn bad() -> &i32 {
    let x = 5;
    &x // Error: `x` does not live long enough

    }

    fn good() -> Box<i32> {
    let x = 5;
    Box::new(x) // Ownership is moved to the caller
    }

    //We can return a pointer/reference only if the data outlives the function, or  by
    //transferring ownership.
    fn get_ref<'a>(input: &'a i32) -> &'a i32 {
    input
    }

 */
fn references() -> i32{
    let mut x = 5;
    let y = &x; // y is a reference to x
    println!("The value of y is: {}", *y);
    //x = 6;
    *y //it returns y value (reference to x)
}

#[cfg(test)]
mod tests {
    use crate::ownership::{cache, go, references};

    #[test]
    fn ownership_test() {
        go();
        cache(&1, &mut 2);
        println!("{}", references());
    }
}