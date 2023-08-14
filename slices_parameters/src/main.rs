fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    println!("word {}", word);
    let word = first_word(&my_string[..]);
    println!("word {}", word);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);
    println!("word {}", word);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("word {}", word);
    let word = first_word(&my_string_literal[..]);
    println!("word {}", word);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("word {}", word);
}


/*
 OJO AQUÍ:
    - La firma de la función permite usar tanto String (complex) como &str (literal) como argumento.
    - Si tenemos un string slice, podemos pasarlo directamente
    - Si tenemos un String, podemos pasar un slice del String o una referencia
*/
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
