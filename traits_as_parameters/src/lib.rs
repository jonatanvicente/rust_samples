

pub trait Summary { //definición
fn summarize(&self) -> String {//método declarado
    String::from("(Read more...)")
}
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle { //impl de Trait
fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.headline, self.author, self.location)
}
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {  //impl
fn summarize(&self) -> String {
    format!("{}: {}", self.username, self.content)
}
}