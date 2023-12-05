/*

   SEgún definimos el lifetime aquí, una instancia de ImportantExcerpt no puede sobrevivir a la
   referencia que contiene en su campo part.
 */

struct ImportantExcerpt<'a> { //definimos lifetime para usar en el cuerpo....
	  part: &'a str,   //añadimos lifetime a la referencia
}

fn main() {
	  let novel = String::from("Call me Ishmael. Some years ago...");
	  let first_sentence = novel.split('.').next().expect("Could not find a '.'"); //manejo de panic
	  
	  let i = ImportantExcerpt {
			part: first_sentence,
	  };
	  
	  /* Aunque novel está creada antes, no sale de alcance hasta que ImportantExcerpt sale de alcance, por lo que la
	  referencia en ImportantExcerpt es válida.
	   */
	  println!("{}", i.part);
}