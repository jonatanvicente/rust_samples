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
    //
}