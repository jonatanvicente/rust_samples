pub fn noalias(input: &i32, output: &mut i32) -> i32{
    if *input == 1 {
        *output = 2;
    } else {
        *output = 3;
    }
    println!("Output is {:}", output);
    *output
}


pub fn mutable_references() -> i32 {
    let x = 42;
    let k = 99;

    let mut y = &x; //WATCH OUT: Mutable Reference, y is of type &i32
    //y = 45;  //error: y is a reference, cannot assign a value directly to it
    y = &45;   //correct: changing the reference to a new reference
    y = &k;    //correct
    println!("y is {:}", y); //output: 99

    //let z = &mut y;       //z is of type &mut &i32
    let z = &y;      //correct, if we will
    //y = &101;             //error: y is borrowed
    // let mut z = y;  //correct, if we will
    println!("z is {:}", z); //output: 99
    //z = 56; //error: cannot assign a value directly to z: it's a mutable reference to a reference
    //z = &56;  //error: z is immutable
    //z = &x;   //error: z is immutable
    *y
}


fn replace_with_new_value(s: &mut Box<i32>) -> i32{

    // this is not okay, as *s would be empty:
    // let was = *s;                       //error: cannot move

    // but this is:
    let was = std::mem::take(s); //correct: take the value
    println!("was: {}", was);
    /*
       It's equivalent to std::mem::replace(&mut value, Default::default());
       it moves value out from behind the mutable reference but leaves a new, default value for the type in its place.
     */
    // so is this:
    *s = was;  //correct
    println!("s: {}", *s);
    *s = Box::new(100);  //correct: replace the value with a new one

    // we can exchange values behind &mut:
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
    *r
}


#[cfg(test)]
mod tests {
    use crate::mut_references::{mutable_references, noalias, replace_with_new_value};

    #[test]
    fn noalias_test() {
        assert_eq! (2, noalias(&1, &mut 1));
        assert_eq! (3, noalias(&2, &mut 2));
        assert_eq! (3, noalias(&3, &mut 3));
        assert_eq! (3, noalias(&4, &mut 5));
        assert_eq! (3, noalias(&6, &mut 7));
        assert_eq! (3, noalias(&8, &mut 9));
    }

    #[test]
    fn mutable_references_test() {
        assert_eq! (99, mutable_references());
    }

    #[test]
    fn replace_with_new_value_test() {
        let mut s = Box::new(42);
        assert_eq!(100, replace_with_new_value(&mut s));
    }
}