use rand::random;

fn lifetimes () {

    let mut x = Box::new(42);
    //take a reference to x
    let r = &x;      // 'a
    if random::<f64>() > 0.5 {
        *x = 84; //requires a &mut x (correct)
    } else {
        println!("{}", r); // 'a
    }
}


fn lifetime_holes() {
    let mut x = Box::new(42);
    //takes a reference to x
    let mut z = &x;   // 'a

    for i in 90..100 {
        println!("{}", z);      // End of 'a
        //moves out of x, x is no longer valid here
        //Borrow checker allows next statement
        x = Box::new(i);
        //when x is moved, z stops existing. Reassign z, we are creating an entirely new variable
        z = &x;                 // 'a
    }
    println!("{}", z);          // 'a
}


///////  Lifetime Variance

struct MutStr<'a, 'b> {//two lifetimes (seems unnecesary)
    s: &'a mut &'b str
}
fn lifetime_variance() {
    let mut s = "hello";

    //next sentence is equivalent to defining a variable x holding a MutStr, and then writing *x.s = "world"...
    // except that there's no variable and so the MutStr is dropped immediately
    *MutStr { s: &mut s }.s = "world";
    println!("{}", s);
}

#[cfg(test)]
mod tests {
    use crate::lifetimes::{lifetime_holes, lifetime_variance, lifetimes};

    #[test]
    fn lifetimes_test() {
        lifetimes();
        lifetime_holes();
        lifetime_variance();
    }
}