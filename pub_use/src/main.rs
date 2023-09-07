use rand::Rng;
use pub_use::hosting;

/*
     Importaciones equivalentes:
        use std::cmp::Ordering;
        use std::io;

      se traduciría en:
        use std::{cmp::Ordering, io};


      Otro ejemplo:
        use std::io;
        use std::io::Write;

      se traduciría en:
        use std::io::{self, Write};

      o se podría
      use std::collections::*;

 */


fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {}", secret_number);

    hosting::add_to_waitlist();
}