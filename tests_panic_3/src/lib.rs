
/*
Retornando error en lugar de panicking
 */

//WATCH OUT we can’t use the #[should_panic] annotation on tests that use Result<T, E>
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())  //retornamos ok si el test pasa
        } else {
            Err(String::from("two plus two does not equal four"))  //... mensaje de error si no
        }
    }
}