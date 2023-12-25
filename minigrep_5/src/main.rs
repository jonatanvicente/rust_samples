use std::{env, fs, process};
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);
    
    
    /*
         - Uso de if let en lugar de unwrap_or_else. Chequea si run retorna un Err value y llama a process::exit(1) si lo hace.
         - Función run no retorna un valor que queramos unwrap de la misma manera que Config::build retorna una instancia de Config.
         - Run retorna () si todo fue bien, por lo que sólo nos preocuparemos de detectar error. No necesitamos unwrap_or_else para
            devolver el unwrapped value, el cual sólo será () aquí.
            El cuerpo de if let y unwrap_or_else son los mismos en ambos casos: imprimimos el error y exit.
    
     */
    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
    
}

/*

fn run(config: Config) {
    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");
    
    println!("With text:\n{contents}");
}

En lugar de permitir que el programa llame a panic! por llamar a expect (función anterior), retornaremos un Result <T, E> si algo fue mal.
Esto nos permitirá manejar el error de una manera más elegante:
       - Cambiamos tipo de retorno
       - Objeto Box<dyn Error> (dyn = dynamic) implementa Error trait (no hace especificar tipo de retorno que devolverá)
       - En lugar de expect -> añadimos operador ? (retornará el error value para ser manejado).
       - Retornamos Ok(()) -> Ok(()) es una forma de decir que no hay nada que devolver, fue llamada para sus side efects solamente. No retorna
          ningún valor que sea necesario.
 */
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    
    println!("With text:\n{contents}");
    
    Ok(())
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        
        let query = args[1].clone();
        let file_path = args[2].clone();
        
        Ok(Config { query, file_path })
    }
}


