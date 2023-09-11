fn main() {

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");

    /*
    Get method returns an Option<&V>; if there’s no value for that key in the hash map, get will return None.
    This program handles the Option by calling copied to get an Option<i32> rather than an Option<&i32>,
    then unwrap_or to set score to zero if scores doesn't have an entry for the key.
     */
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score: {}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}
