

pub fn go(){
    let x = 42;
    let y = 43;
    let var1 = &x;
    println!("var1 = {:?}", var1);
    let mut var2 = &x;
    println!("var2 = {:?}", var2);
    var2 = &y;
    println!("var2 = {:?}", var2);
}


pub fn flows(){
    let mut x;
    // this access would be illegal, nowhere to draw the flow from:
    // assert_eq!(x, 42);
    x = 42;
    // this is okay, can draw a flow from the value assigned above:
    let y = &x;  //'x' is borrowed here
    // this establishes a second, mutable flow from x:

    //x = 43; //WATCH cannot assign x, is borrowed
    /*
        - WATCH: You cannot have an exclusive and a shared use of a value at the same time
        - Several variables can point to the same value.
        - The value is borrowed, it cant be changed.
     */
    let k = &x;
    // this continues the flow from y, which in turn draws from x.
    // but that flow conf
    assert_eq!(*y, 42);
    assert_eq!(*k, 42);
}


pub fn low_level() {
    let mut x: usize; //x is the name for a region of memory on the stack (its slot is empty)
    x = 6; // that region hold the bits to value 6
    let y = &x; //does not change when you assign to x
    assert_eq!(*y, 6);
    // x = 22;//is borrowed here, so you cannot change it
}


#[cfg(test)]
mod tests {
    use crate::pointers::{flows, go, low_level};

    #[test]
    fn pointers_test() {
        go();
        flows();
        low_level();
    }
}