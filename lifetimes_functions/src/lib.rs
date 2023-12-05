

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


/*
    Si, en lugar de lo anterior, deseamos que longest retorne siempre el primer parámetro en lugar del más largo,
    no necesitaremos especificar el lifetime en el segundo parámetro y.
    Lo especificaremos para x (lifetime 'a) y para el valor de retorno (lifetime 'a), pero no para y porque no
    tiene relación con el lifetime de x o del valor de retorno.
*/
fn longest_2<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

/*
    OJO: cuando retornomos una referencia, el lifetime del retorno debe coincidir con el lifetime de uno de los parámetros.
    Si la referencia retornada no apunta a uno de los parámetros, estaría referida a un valor creado dentro de la función,
    lo cual podría crear un puntero colgante. Ejemplo:
    
        fn longest<'a>(x: &str, y: &str) -> &'a str {
            let result = String::from("really long string");
            result.as_str()
            }

     FAlla compile: el lifetime de retorno no tiene que ver con los parámetros de entrada

*/