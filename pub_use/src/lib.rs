mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting; // Making a name available for any code to use from a new scope with pub use

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}