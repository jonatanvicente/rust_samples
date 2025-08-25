/*
    aggregator crate contains the Summary trait and Tweet struct
 */

use aggregator::{Summary, Tweet};

#[doc = "This function shows how to use traits using aggregator crate"]
//#[doc(hidden)] //Lets you hide a public item from your documentation without making it inaccessible to code that happens to know it is there.
fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize()); //muestra resumen de data existente, pertenece a aggregator

}
/*

Next three functions all take a string and return a string, but there are some differences.
None of these function signatures is better than the others:

    1.- Caller owns the string, and the function will return an owned String:

        fn function1(s: String) -> String

    2.- Caller provides a reference: if he has a String, he must dereference it to a &str.

        fn function2(s: &str) -> Cow<'_, str>

    3.- The user pass in a type that can produce a reference to a string. The return value can produce a reference to a string.

        fn function3(s: impl AsRef<str>) -> impl AsRef<str>

 */


