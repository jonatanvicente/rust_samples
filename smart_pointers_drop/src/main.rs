struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
/*    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~");*/
    main_2();
}

fn main_2() {
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    println!("{}", c.data);
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
    //println!("{}", c.data);//err: value used after moved
}
