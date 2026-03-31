

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

---

### The Challenge: Galactic Logistics System

#### Phase 1: The Foundations (Structs & Enums)

First, define the **core data types**. This will test your ability to nest structures and use Enums for state.
1. **CargoCategory** (Enum): Should have variants like Mineral, Medicine, and Technology.
2. **CargoItem** (Struct): Needs a name, a category, and a price_per_unit (f64).
3. **Ship** (Struct): Needs a name, a capacity (u32), and a cargo_hold (a Vec of CargoItem).
4. **SpaceStation** (Struct): Needs a name and a docked_ships list.

 
#### Phase 2: The Borrowing Gauntlet

Implement the following functions. Pay close attention to the function signatures—this is where the borrowing logic lives.
- **Task A**: The Manifest (Immutable Borrowing)
    - Write a function **print_ship_manifest** that takes a reference to a Ship and prints every item in its hold.
    - Constraint: The ship must not be destroyed (dropped) after printing.
- **Task B**: The Quality Check (Mutable Borrowing)
    - Write a function **apply_space_wear** that takes a mutable reference to a CargoItem. It should reduce the price_per_unit by 10% due to "radiation damage."
- **Task C**: Docking (Moving Ownership)
    - Write a function **dock_ship** that takes a SpaceStation and a Ship, and adds the ship to the station's docked_ships list.
    - Critical Question: Should the station own the ship, or just borrow it? (For this exercise: the station should take ownership).

#### Phase 3: Complex Logic (The "Sticky" Situations)

This is where most beginners hit a wall with the borrow checker.
1. **The Transfer Logic:** Implement a function transfer_cargo(source: &mut Ship, destination: &mut Ship, item_name: &str).
    - Find the item in the source ship by name.
    - Remove it from the source (moving it out of the vector).
    - Push it into the destination ship.
    - Hint: You will need to handle the case where the item isn't found.
2. **The Audit:**
    - Implement a function calculate_total_value(station: &SpaceStation) -> f64.
    - It must iterate through every ship, and every item in every ship.
    - Formula:$$TotalValue = \sum_{ships} \sum_{items} (price\_per\_unit)$$
    - Constraint: You must do this without taking ownership of the station or the ships.

#### Phase 4: Refactoring for Safety

Once you have the logic, try to break it to see what the compiler says:
    - The "Double Borrow" Trap: Try to write a loop that iterates over a ship's cargo (&ship.cargo_hold) and, inside that loop, tries to call apply_space_wear on the same item.
    - The "Dangling Reference" Trap: Try to write a function that searches for the most expensive item in a ship and returns a reference to it. Then, try to clear the ship's cargo and see if you can still use that reference.