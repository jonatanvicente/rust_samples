
pub trait Summary { //definición
fn summarize_author(&self) -> String;
    
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())  //definido el comportamiento por default del método
        //default implementations pueden llamar a otros métodos del trait, aunque éstos no estén implementados
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet { //OJO no es necesario cambiar la implementación aquí, aunque hayamos cambiado la definición arriba
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}