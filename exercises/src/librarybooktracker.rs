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
}

fn print_library_status(books: &[book]){

    for book in books {
        println!("Name: {}, is available: {}", book.title, book.is_available);
    }

}