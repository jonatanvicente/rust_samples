#![allow(unused)]
use crate::List::{Cons, Nil};

fn main() {
      store_data_on_heap();
      store_const_list();
}


fn store_data_on_heap(){
      let b = Box::new(5); //5 está en heap
      println!("b = {}", b);
      
      /*
		   Podemos acceder a la variable en heap similar a como está en stack. Cuando el Box sale del scope, el valor en heap será eliminado.
		   Los valores serán eliminados para el Box (en Stack) como para el lugar donde apunta (heap).
		   Cuando Box sale de scope, el valor en heap será eliminado porque implementa Drop trait.
	   */
}


enum List {
      //Box es un puntero a un valor en heap. Rust siempre sabe cuánto ocupa el puntero, pero no el valor al que apunta; Lo colocamos en Cons
      //en lugar de almacenar valor con espacio infinito, con box no lo es
      Cons(i32, Box<List>),
      Nil,
}

fn store_const_list(){
      
      //Cada box apuntará a la lista siguiente
      let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
