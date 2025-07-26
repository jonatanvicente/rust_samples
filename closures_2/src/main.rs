use std::thread;
use std::time::Duration;

#[allow(unused_variables)]

fn main() {
// Adding optional type annotations of the parameter and return value types in the closure
	  let expensive_closure = |num: u32| -> u32 {
			println!("calculating slowly...");
			thread::sleep(Duration::from_secs(2));
			num
	  };
	  
	  
	  //Función simple
	  fn  add_one_v1   (x: u32) -> u32 { x + 1 }
	  
	  //Closures equivalentes (hay inferencia de tipos (??))
	  let add_one_v2 = |x: u32| -> u32 { x + 1 };//Closure completamente anotada
	  let add_one_v3 = |x: u32|             { x + 1 };
	  let add_one_v4 = |x: u32|               x + 1  ;//Ninguna anotación
	  
}
