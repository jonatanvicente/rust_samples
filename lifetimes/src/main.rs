/*fn main() {
	  let r;
	  
	  {
			let x = 5;
			r = &x; //r apunta a x en este scope
	  }
	  
	  println!("r: {}", r); //r es null (no permitido en Rust)
}*/


//este código sí es correcto (misma scope)
fn main() {
	  let x = 5;            // ----------+-- 'b
	  //           |
	  let r = &x;           // --+-- 'a  |
	  //   |       |
	  println!("r: {}", r); //   |       |
	  // --+       |
}                         // ----------+