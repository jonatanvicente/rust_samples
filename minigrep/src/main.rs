use std::env; //env module
//std::env::args_os si necesitamos capturar caracteres unicode inválidos


//cargo run -> sin args
//cargo run -- needle haystack -> con args (ejemplo)
fn main() {
    //capture command line args. Retorna un Iterator sobre commands recibidos
    //collect crea la colección. Indicamos tipo (OJO no se inferirá tipo si no lo ponemos)
    
    let args: Vec<String> = env::args().collect();
    //dbg!(args); //imprimimos con debug macro
    
    let query = &args[1];
    let file_path = &args[2];
    
    println!("Searching for {}", query);
    println!("In file {}", file_path);
}

/*
  OUTPUT:
  [src/fibonacci:5] args = [
    "target/debug/minigrep",
]
    - OJO primer elemento del vector -> binario que ejecutamos
 */