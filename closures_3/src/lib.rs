use std::thread;

pub fn closure_definition(){
	  //vector
	  let list = vec![1, 2, 3]; //immutable
	  println!("Before defining closure: {:?}", list);
	  
	  //definición de closure enlazada con una variable
	  let only_borrows = || println!("From closure: {:?}", list);
	  println!("Before calling closure: {:?}", list);
	  
	  //invocación de la closure (misma forma que una función)
	  only_borrows();
	  println!("After calling closure: {:?}", list);
}

pub fn closure_mut(){
	  let mut list = vec![1, 2, 3]; //OJO mut
	  println!("Before defining closure: {:?}", list);
	  
	  let mut borrows_mutably = || list.push(7); //first borrow
	  
	  //ERROR: cannot borrow `list` as immutable because it is also borrowed as mutable
	  //println!("Before calling closure: {:?}", list);//immutable to print: not allowed here
	  
	  borrows_mutably();
	  println!("After calling closure: {:?}", list);
}

/*
	Si deseamos forzar la closure para que tome el ownership de los valores (aunque el cuerpo no necesita
	estrictamente ownership), podemos usar move antes de la lista de params.
	Esto es muy usado cuando pasamos una closurea a un nuevo thread para mover la data y que sea propiedad
	del nuevo thread.
 */

pub fn closure_move(){
	  
	  let list = vec![1, 2, 3];
	  println!("Before defining closure: {:?}", list);
	  
	  /*
	       Generamos nuevo hilo: pasamos una closure.
	       Aunque el cuerpo de la closure sólo necesita una immutable reference, necesitamos especificar que la
	       lista debería ser trasladada a la closure al poner move antes.
	       El nuevo thread podría terminar antes o después que el thread principal.
	       Si el thread principal mantuviera la propiedad de la lista pero terminara antes de que lo hiciera el nuevo hilo y eliminara la lista,
	       la referencia inmutable en el hilo no sería válida.
	       Por lo tanto, el compilador requiere que la lista se mueva a la closure dada al nuevo thread para que la referencia sea válida.
	       
	   */
	  thread::spawn(move || println!("From thread: {:?}", list))
		  .join()
		  .unwrap();
}

/*
	IMPLEMENTACIÓN DE TRAIT BOUNDS POR LAS CLOSURES.
	Definición de qué ocurrirá con los valores obtenidos en la closure.
	Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:
	    - FnOnce -> para closures que son llamadas una vez. Todas las closures implementan este trait como mínimo. Si sólo es llamada una vez, no implementará
	      ninguno de los otros traits
	    - FnMut -> para closures que no mueven los valores capturados fuera del body, pero que podrían mutarlos. Pueden ser llamadas más de una vez.
	    - Fn -> closures que no mueven los valores capturados fuera del body y no mutan los valores capturados (igual que closures que no capturan nada).
	      Pueden ser llamadas más de una vez sin mutar su entorno (importante en casos de llamaadas a una closure multiples veces concurrentemente).
	
	Note: Functions can implement all three of the Fn traits too. If what we want to do doesn’t require capturing a value from the environment,
	we can use the name of a function rather than a closure where we need something that implements one of the Fn traits.
	For example, on an Option<Vec<T>> value, we could call unwrap_or_else(Vec::new) to get a new, empty vector if the value is None.
 */

/*

  Definición del método unwrap_or_else (del core):
 
//T -> generíco que representa el tipo de valor en Some de Option
impl<T> Option<T> {
	  
	  //Retorno -> mismo tipo T
	  //F -> tipo de f, que es la closure que proveemos cuando llamamos unwrap_or_else
	  pub fn unwrap_or_else<F>(self, f: F) -> T
	  //F podrá ser llamada como máximo una vez sin argumentos y retornar un T
			where
				F: FnOnce() -> T  //todas las closures implementan este trait como mínimo
	  {
			match self { //si la Option es Some -> f no será llamada
				  Some(x) => x,
				  None => f(), //si es None -> f será llamada una vez
			}
	  }
}

*/

#[derive(Debug)]
struct Rectangle {
	  width: u32,
	  _height: u32,
}

//Modificación respecto al ejemplo anterior: uso de FnMut en lugar de FnOnce
pub fn closure_fnmut() {
	  let mut list = [
			Rectangle { width: 10, _height: 1 },
			Rectangle { width: 3, _height: 5 },
			Rectangle { width: 7, _height: 12 },
	  ];
	  
	  /*
	       Aqui la closure obtiene un argumento en forma de referencia al item actual de la lista.
	       Esta función es usable cuando queremos ordenar un slice por un att particular de cada item.
	       Se ordena por el width de cada item de menor a mayor.
	   */
	  list.sort_by_key(|r| r.width); //FnMut
	  /*
	      FnMut: llamada múltiples veces. La closure |r| r.width no captura, muta o mueve nada fuera de su environment,
	      por lo que cumple con los requerimientos de FnMut.
	   */
	  println!("{:#?}", list);
}


pub fn closure_fnmut_2() {
	  let mut list = [
			Rectangle { width: 10, _height: 1 },
			Rectangle { width: 3, _height: 5 },
			Rectangle { width: 7, _height: 12 },
	  ];
	  
	  let mut num_sort_operations = 0;
	  list.sort_by_key(|r| {
			num_sort_operations += 1; //capturamos la ref mutable para sacarla del ámbito. La closure podrá ser llamada más de una vez
			r.width
	  });
	  println!("{:#?}, sorted in {num_sort_operations} operations", list);
}



pub fn closure_fnonce(){

}