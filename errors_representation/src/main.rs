use std::fs::File;
use std::error::Error;
/*
Debug should provide a more descriptive error including auxiliary information that may be useful
in tracking down the cause of the error, such as port numbers, request identi-
fiers, filepaths, and the like, which #[derive(Debug)] is usually sufficient for.
*/

//To distinguish between different kinds of errors, we can use an enum to represent them
#[derive(Debug)]
pub enum CopyError {
    In(std::io::Error),
    Out(std::io::Error),
}


/*

     Where possible, your error type should be 'static:
        - That it allows the caller to more easily propagate your error up the call stack without
            running into lifetime issues.
        - It also enables your error type to be used more easily with type-erased error types, as we’ll see shortly.

     Errors are often placed behind a pointer type, such as a Box or Arc. This way, they’re unlikely
     to add much to the size of the overall Result type they’re contained within.

 */


fn do_the_thing() -> Result<(), Box<dyn Error>> {
    let thing = File::open("hello.txt")?;
    // .. code that uses thing and ? ..
    let _ = thing.lock();
    Ok(())
}

fn main() {
    let _ = do_the_thing(); //using _ to ignore the resulting value
}
