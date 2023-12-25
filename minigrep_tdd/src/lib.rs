use std::error::Error;
use std::fs;

pub struct Config {
	  pub query: String,
	  pub file_path: String,
}

#[allow(unused_variables)]
impl Config {
	  pub fn build(args: &[String]) -> Result<Config, &'static str> {
			if args.len() < 3 {
				  return Err("not enough arguments");
			}
			
			let query = args[1].clone();
			let file_path = args[2].clone();
			
			Ok(Config { query, file_path })
	  }
}

#[allow(unused_variables)]
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
	  let contents = fs::read_to_string(config.file_path)?;
	  
	  for line in search(&config.query, &contents) {
			println!("{line}");
	  }
	  Ok(())
}

//We need to define an explicit lifetime 'a in the signature of search and use that lifetime with the contents argument and the return value
//We tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument.
#[allow(unused_variables)]
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
	  
	  let mut results = Vec::new();
	  
	  for line in contents.lines() {
			if line.contains(query) {
				  results.push(line);
			}
	  }
	  results
}


#[cfg(test)]
mod tests {
	  use super::*;
	  
	  #[test]
	  fn one_result() {
			let query = "duct";
			//the backslash after the opening double quote tells Rust not to put a newline character at the beginning of the contents of this string literal
			let contents = "\
Rust:
safe, fast, productive.
Pick three.";
			
			assert_eq!(vec!["safe, fast, productive."], search(query, contents));
	  }
}


