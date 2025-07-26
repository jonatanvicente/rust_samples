use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { //nueva instancia de Pair<T>
        Self { x, y }
    }
}

/**
   IMPLEMENTACIÓN CONDICIONAL:
        - Podemos condicionalmente implementar un trait para cualquier tipo que implemente otro trait.
        - En el ejemplo siguiente (Standard library), el trait ToString es implementado en cualquier tipo que implemente Display trait.

                impl<T: Display> ToString for T {
                    // --snip--
                }
*/

impl<T: Display + PartialOrd> Pair<T> {

    //método cmp_display sólo se implementará si T implementa Display (mostrado en pantalla) y PartialOrd (comparaciones)
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


