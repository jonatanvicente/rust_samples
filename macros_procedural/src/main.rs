
//simple macro to implement Debug
macro_rules! name_as_debug {
    ($t:ty) => {

        use core::fmt::{Debug, Formatter, Result, Write};
        use core::stringify;

        impl Debug for $t {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result{
                    write!(f, stringify!($t))
                }
            }
        };
    }


struct MyType;

fn main() {
    println!("Hello, world!");
    name_as_debug!(MyType);
}
