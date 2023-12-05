struct ImportantExcerpt<'a> {
    part: &'a str,
}

//aquí necesitamos identificar el lifetime despues de impl y entonces usarlo tras el nombre
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { //aquí no es necesario especificar el lifetime porque ya lo hicimos en la definición de la estructura
        3
    }
}

//ejemplo donde aplica la 3ª regla elision
impl<'a> ImportantExcerpt<'a> {
    //Rust aplica la 1ª elision rule: da a &self y announcement su propio lifetime
    //ya que uno de los parámetros es &self, el lifetime del retorno será el mismo que el de &self
    fn announce_and_return_part(&self, announcement: &str) -> &str { //2 input lifetimes, 1 output lifetime
        println!("Attention please: {}", announcement);
        self.part
    }
}


//~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//Ejemplo uso de self
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}