

//DOES NOT COMPILE: el puntero de retorno no puede saber el ámbito de las variables que recibe en firma
/*
    
    fn longest(x: &str, y: &str) -> &str {
	  if x.len() > y.len() {
			x
	  } else {
			y
	  }
    }

     En su lugar, fijamos la siguiente firma:
     Queremos que el valor de retorno sea válido tanto tiempo como los parámetros de entrada lo sean.
     OJO: la anotación de lifetime NO cambia el lifetime de los parámetros; tan sólo indica al borrow checker
     que rechace todos los valores que no se adhieran a esta constraint.
}
 */

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}



