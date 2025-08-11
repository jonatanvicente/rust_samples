pub fn go() {
    let x1 = 42;
    let y1 = Box::new(84);
    { // starts a new scope
        let z = (x1, y1);
        // z goes out of scope, and is dropped;
        // it in turn drops the values from x1 and y1
    }
    // x1's value is Copy, so it was not moved into z
    let x2 = x1;
    // y1's value is not Copy, so it was moved into z
    // let y2 = y1;

}

#[cfg(test)]
mod tests {
    use crate::ownership::go;

    #[test]
    fn ownership_test() {
        go();
    }
}