fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);//referimos al VALOR de s1, pero no nos hacemos owners de él
    println!("The length of '{}' is {}.", s1, len);

    //change(&s1);//no podemos modificar s1 porque es inmutable por default
}


//recibimos parámetro como referencia, s is a reference to a String
/*
Cuando las funciones tienen referencias como parámetros en lugar de los valores reales,
no necesitaremos devolver los valores para devolver la propiedad, porque nunca tuvimos la propiedad.
 */
fn calculate_length(s: &String) -> usize {//usize es tipo de retorno: integer de longitud dependiente de la arquitectura, unsigned
    s.len()
}
//el valor adonde apuntamos no será dropped cuando la referencia deje de usarse
// Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.


/*fn change(some_string: &String) {
    some_string.push_str(", world");//no podemos modificar s1 porque es inmutable por default
}*/

