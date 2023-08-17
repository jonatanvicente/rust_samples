#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

/*
Another way to print out a value using the Debug format is to use the dbg! macro, which takes
ownership of an expression (as opposed to println!, which takes a reference),
prints the file and line number of where that dbg! macro call occurs in your code along with
the resultant value of that expression, and returns ownership of the value.
 */
fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),//retorna el ownership del value
        height: 50,
    };

    dbg!(&rect1);
}