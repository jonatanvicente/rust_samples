use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    /*
        unwrap_or_else nos permite manejo de error en lugar de con panic!:
            - Si Result es OK -> el comportamiento es igual a unwrap(): retorna el valor dentro de Ok
            - Si no (hay un Err value) -> llama al código en el closure (funcion anonima que pasamos como arg a unwrap_or_else)
        unwrap_or_else pasará el valor interno de Err (en este caso el static string "not enough arguments") al closure en el
        argumento err que aparece entre | |. El codigo en el closure puede usar el err value cuando retorna.
     */
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    /*
       Build function retorna  una instancia de Config si todo fue bien y una &'static str si no.
       Mensajes de error serán siempre string literals con lifetime 'static.
       En lugar de llamar a panic!, retornamos mensaje de error. Esto permite manejar a main mejor el mensaje
     */
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { query, file_path })
    }
}


