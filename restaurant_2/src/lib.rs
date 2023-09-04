mod front_of_house {//inicio cuerpo del módulo

    pub mod hosting { //necesario pub para llamar desde abajo a add_to_waitlist()
        pub fn add_to_waitlist() {} //idem

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    // Relative path - eat_at_restaurant and front_of_house are siblings, we can refer to front_of_house from eat_at_restaurant
    front_of_house::hosting::add_to_waitlist();
}