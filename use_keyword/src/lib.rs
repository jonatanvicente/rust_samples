use std::fmt;
use std::io;
//use std::io::Result as IoResult; //otra posibilidad: alias para evitar conflictos de nombres

mod front_of_house {
    pub mod hosting {//módulo importado abajo
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; //sólo creada para uso particular del scope. No funcionará si lo incluimos en otro scope
//use crate::front_of_house::hosting::add_to_waitlist; //preferible importación superior,
// habilitando  el módulo y no sólo la función con full path



//mod customer { //inhabilitará la llamada, use no podrá ser importado
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
//}


//  Ejemplo de cómo referenciar a tipos con el mismo nombre

fn function1() -> fmt::Result {
    return Ok(());
}

fn function2() -> io::Result<()> {
    return Ok(());
}