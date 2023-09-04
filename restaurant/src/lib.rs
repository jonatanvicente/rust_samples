//definimos el módulo con mod, seguido del nombre

mod front_of_house {//inicio cuerpo del módulo

    /*
        Un módulo puede contener otros módulos,
        structs, enums, constants, traits y functions
     */

    /*
        Tampoco funcionaría el acceso de abajo a las funciones privadas del módulo hosting si pusiésemos:
            pub mod hosting
        - Esto haría el módulo interno público. Podríamos acceder a hosting si alcanzamos el módulo front_of_house,
          pero el contenido de hosting sigue siendo privado.
        - HACER EL MÓDULO PÚBLICO NO HACE SU CONTENIDO TAMBIÉN público.
        - Ya que los módulos son containers, no conseguimos mucho haciendo público el módulo solamente. Necesitamos hacer también
          público el contenido del módulo.

     */
    //pub mod hosting { //necesario para llamar desde abajo a add_to_waitlist()
    mod hosting {
        // OJO: The privacy rules apply to structs, enums, functions, and methods as well as modules.
        fn add_to_waitlist() {}
        // pub fn add_to_waitlist() {} // esto sí permitiría el acceso desde eat_at_restaurant()

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

/*
    El módule tree será:

            crate
             └── front_of_house
                 ├── hosting
                 │   ├── add_to_waitlist
                 │   └── seat_at_table
                 └── serving
                     ├── take_order
                     ├── serve_order
                     └── take_payment
 */


pub fn eat_at_restaurant() {
    // Absolute path
    //crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    //front_of_house::hosting::add_to_waitlist();

    /*
         No compila ninguna de las dos definiciones, ni absolute ni relative: add_to_waitlist es private.
         Rust no puede acceder a secciones privadas. Todos los items son privados a los parent modules por default.
         Si queremos hacer un item como function o struct privado, debemos ponerlo en un módulo.

         * Parent no pueden usar items privados de child modules, pero OJO child modules SÍ pueden
            usar items privados de parent modules.
         * Si deseamos exponer partes internas de un child module, usaremos pub keyword para hacer un item público.

     */
}