enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


//véase https://doc.rust-lang.org/std/vec/struct.Vec.html
fn main() {
    let row = vec![ //definimos vector de diferentes tipos
                    SpreadsheetCell::Int(3),
                    SpreadsheetCell::Text(String::from("blue")),
                    SpreadsheetCell::Float(10.12),
    ];

    //Ejemplos de inicialización de vectores (ver doc)
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);
    vec[0] = 7;
    assert_eq!(vec[0], 7);
    vec.extend([1, 2, 3]);
    for x in &vec {
        println!("{x}");
    }
    assert_eq!(vec, [7, 1, 2, 3]);

    //
    let mut vec1 = vec![1, 2, 3];
    vec1.push(4);
    let vec2 = Vec::from([1, 2, 3, 4]);
    assert_eq!(vec1, vec2);


    //
    let vec = vec![0; 5];
    assert_eq!(vec, [0, 0, 0, 0, 0]);

// The following is equivalent, but potentially slower:
    let mut vec = Vec::with_capacity(5);
    vec.resize(5, 0);
    assert_eq!(vec, [0, 0, 0, 0, 0]);
    //
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        // Prints 3, 2, 1
        println!("{top}");
    }

    // Salida de scope
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here


}



