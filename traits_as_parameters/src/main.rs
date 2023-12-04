use std::fmt::{Debug, Display};
use std::ptr::null;
use aggregator::{Summary, Tweet};

fn main() {
	  let tweet = Tweet {
			username: String::from("horse_ebooks"),
			content: String::from(
				  "of course, as you probably already know, people",
			),
			reply: false,
			retweet: false,
	  };
	  
	  println!("1 new tweet: {}", tweet.summarize()); //muestra resumen de data existente, pertenece a aggregator
	  
}

pub fn notify(item: &impl Summary) {
	  println!("Breaking news! {}", item.summarize());
}

pub fn notify_2<T: Summary>(item: &T) { //OJO paso de parámetros, equivalente al enterior
	  println!("Breaking news! {}", item.summarize());
}

pub fn notify_3(item: &(impl Summary + Display)) {} //recibimos dos implementaciones de trait

pub fn notify_4<T: Summary + Display>(item: &T) {}


/*
    Simplificación de signature:
    Para hacer más legible
    
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}
    
    podemos establecer...
 */

fn some_function<T, U>(t: &T, u: &U) -> i32
	  where
		  T: Display + Clone,
		  U: Clone + Debug,
{ return i32::new }

//retorno de Traits
fn returns_summarizable() -> impl Summary {
	  Tweet {
			username: String::from("horse_ebooks"),
			content: String::from(
				  "of course, as you probably already know, people",
			),
			reply: false,
			retweet: false,
	  }
}