use std::fmt::Display;


/*
   Retorna la cadena más larga.
   Recibe (además de las cadenas), un parámetro ann de tipo T, el cual debe implementar Display (cláusula Where).
   El parámetro ann será impreso por pantalla, lo cual indica porqué es necesario el trait Display.
 */
pub fn longest_with_an_announcement<'a, T>( //Los lifetimes son un tipo de genérico, las declaraciones de 'a y T van en la misma lista dentro de los <>
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
    where
        T: Display,
{
    println!("Announcement! {}", ann); //imprimimos ann por pantalla
    if x.len() > y.len() {
        x
    } else {
        y
    }
}