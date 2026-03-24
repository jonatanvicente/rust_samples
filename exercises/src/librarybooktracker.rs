struct book {
    title: String,
    author: String,
    is_available: bool
}

impl book {
    const fn new(title: String, author: String, is_available: bool) -> book {
        book {
            title,
            author,
            is_available: true
        }
    }

    fn check_out(&mut self) {
        if self.is_available {
            self.is_available = false;
            println!("You have checked out '{}'", self.title);
        } else {
            println!("Sorry, '{}' is currently unavailable.", self.title);
        }
    }
}

pub fn go(){
    let book_a = book::new("Don Quijote".to_string(), "Miguel de Cervantes".to_string(), true);
    let book_b = book::new("The Raven".to_string(), "Edgar Allan Poe".to_string(), true);
    let book_c = book::new("Moby Dick".to_string(), "Herman Melville".to_string(), true);

    let mut books: Vec<book> = Vec::new();
    books.push(book_a);
    books.push(book_b);
    books.push(book_c);

    print_library_status(&books);

    /*
        - We are calling IntoIterator::into_iter(books). Tis consumes the vector
        - books is moved into the loop. Each book is moved out of the vector and into the variable book.
        - By default, new var book isn't mut.
        - for book in books is not valid
        - for book in &mut books, you are iterating over mutable references to the items inside the vector.
            No Move Occurs: "Hold onto the vector, just give me a temporary 'key' to change each item inside it one by one."
            Mutable Access: The variable book now has the type &mut Book. Because it is a mutable reference, you are allowed to call methods like change() which likely require &mut self.
            Vector Survives: After the loop, the vector books is still alive and contains all your updated data.
     */

    for book in &mut books{
        book.check_out();
    } //Rust generally assumes that if you're iterating over a collection by value, you're finishing it off, not updating it for later use.

    print_library_status(&books);
}

fn print_library_status(books: &[book]){

    for book in books {
        println!("Name: {}, is available: {}", book.title, book.is_available);
    }

}