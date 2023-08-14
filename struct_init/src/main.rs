struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    //let user1 = User { //hacemos mutable
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    //posible también llamar a build_user
    build_user("email@gmail.com".to_string(), "username".to_string());

    /*
    let user2 = User {
            active: user1.active,
            username: user1.username,
            email: String::from("another@example.com"),
            sign_in_count: user1.sign_in_count,
        };
     */

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 //resto de campos de user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        //username: username,
        username, // field init shorthand
        //email: email,
        email,
        sign_in_count: 1,
    }
}
