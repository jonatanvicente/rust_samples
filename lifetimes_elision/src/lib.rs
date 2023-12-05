/*
    Compila, aunque en versiones previas de RUST no: cada referencia necesita un lifetime
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


/*
    Podría reescribirse así:
 */

fn first_word_2<'a>(s: &'a str) -> &'a str {
    
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

/*
      Fueron añadidas al compilador la lifetimes porque los programadores RUST vez tras vez usaban
      las referencias de la misma manera, de acuerdo a patterns comunes. Por eso el RUST team lo añadió al compilador.
      Estos patrones son llamados lifetime elision rules.
      
      Lifetimes en funciones o parámetros de métodos -> input lifetimes
      Lifetimes en parámetros de retorno -> output lifetimes
      
 */