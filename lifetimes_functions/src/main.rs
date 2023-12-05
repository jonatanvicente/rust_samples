use lifetimes_functions::longest;

fn main() {
	  let string1 = String::from("abcd");
	  let string2 = "xyz";
	  
	  let result = longest(string1.as_str(), string2);
	  println!("The longest string is {}", result);
	  
	  main_2();
	//  main_3();
}

// Utilización de la función con diferentes lifetimes
fn main_2() {
	  let string1 = String::from("long string is long");
	  
	  {
			let string2 = String::from("xyz");
			let result = longest(string1.as_str(), string2.as_str());
			println!("The longest string is {}", result);
	  }
}

/*
fn main_3() {
	  let string1 = String::from("long string is long"); //OJO diferente scope en ambas variables
	  let result;
	  {
			let string2 = String::from("xyz");
			result = longest(string1.as_str(), string2.as_str()); //el lifetime de result será EL MENOR de los ámbitos de ambas variables
	  }
	  println!("The longest string is {}", result);  //inválido: fuera de scope
}

*/



