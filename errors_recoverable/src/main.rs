use std::fs::File;
use std::io::ErrorKind;

fn main() {

    match_error();
    different_errors();
    closures();

}

fn match_error() {
    /*
     The return type of File::open is a Result<T, E>
     The generic parameter T has been filled in by the implementation of File::open with the type of
      the success value, std::fs::File, which is a file handle. The type of E used in the error value
      is std::io::Error. This return type means the call to File::open might succeed and return a file
      handle that we can read from or write to.
     */
    let greeting_file_result = File::open("hello.txt");

    let _greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

}

fn different_errors() {

        let greeting_file_result = File::open("hello.txt");

        let _greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating the file: {:?}", e),
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
        };
    }

fn closures() {
    //unwrap_or_else -> metodo para evitar muchos match anidados
    let _greeting_file = File::open("hello.txt").unwrap_or_else(|error| {

        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}