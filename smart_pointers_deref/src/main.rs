use std::ops::Deref;
use smart_pointers_deref::hello;

fn main() {
    //Comportamiento de referencias por default
    following_pointer_to_value();
    box_like_a_reference();

    //Definición de nuestro propio Smart Pointer
    my_smart_pointer();

    //Deref Coercions con funciones y métodos
    deref_with_mybox();
}


fn following_pointer_to_value(){
    let x = 5;
    let y = &x;//y = reference to x

    assert_eq!(5, x); //x es 5
    /*
        assert_eq!(5, y);//ERROR
        ERROR: y es una referencia a x. Es necesario poner '*' para seguir el valor adonde apunta el puntero (dereferenciar),
        y entonces el compilador podrá comparar los valores.
        Una vez dereferenciado y, tenemos acceso al valor integer que apunta y podemos compararlo.
     */
    assert_eq!(5, *y);
}

//Funcionamiento similar al anterior
fn box_like_a_reference(){
    let x = 5;
    let y = Box::new(x);//y = box to x

    assert_eq!(5, x);
    assert_eq!(5, *y);
}


struct MyBox<T>(T); //Definimos para soportar cualquier tipo

impl<T> MyBox<T> { //tupla con un elemento T.
    fn new(x: T) -> MyBox<T> { //toma el parámetro T y retorna la instancia con el valor T
        MyBox(x)
    }
}

fn my_smart_pointer(){
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);//ERROR MyBox no puede ser dereferenciada. Es necesario implementar el trait Deref (añadido abajo)

}

//Sin el trait Deref, el compilador no sabe cómo dereferenciar MyBox<T> a T. Sólo puede dereferenciar referencias '&'
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


fn deref_with_mybox(){
    let m = MyBox::new(String::from("Rust"));
    hello(&m); //Deref coercion: Rust convierte &MyBox<String> en &String (llamando a deref)

    //Si RUST no hiciese esto, tendríamos que escribir....
    hello(&(*m)[..]); //(*m) dereferencia MyBox<String> a String. & y [..] toman una referencia a un string slice que contiene toda la cadena.
    //No hay penalización alguna en runtime en las llamadas a Deref

}
