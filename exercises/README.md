

### The "Library Book Tracker"
Your goal is to create a small system that manages a list of books and tracks whether they are currently checked out. This will help you practice how Rust handles data movement.

#### 1. Define your Data
Create a Book struct with the following fields:
   - **title**: A String
   - **author**: A String
   - **is_available**: A bool.

#### 2. Implement Methods

Use an impl block for Book to create two methods:
- **new**: A static method (constructor) that takes a title and author and returns a new Book instance (defaulting is_available to true).
- **check_out**: A method that takes a mutable reference to self. It should print a message saying the book is being checked out and flip the is_available flag to false. If it’s already checked out, print a warning.

#### 3. The Library Logic

In your main function:
- Create a Vec (vector) of Book instances.
- Write a function called print_library_status that takes a reference to the vector and prints out each book's title and its availability.
- Borrow a book from the vector and call the .check_out() method on it.
- Call print_library_status again to ensure the state updated correctly.

**TIP:** The Ownership Hurdle: Remember that if you try to loop through your vector using for book in books, you will move the books out of the vector. Use for book in &books to borrow them instead!

**Why this exercise?**
This forces you to distinguish between owning the data (the Vector), borrowing the data (printing the list), and mutably borrowing the data (checking a book out).