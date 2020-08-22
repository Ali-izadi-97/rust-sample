struct User {
    username: String,
    password: String,
    active: bool,
}

fn main() {

    let user1 = User {
        password: String::from("make me"),
        username: String::from("username"),
        active: true
    };

    let user2 = create_user("ali", "toole");

    println!("Hello, world! {}", user2.username);
}


fn create_user(username: &str, pass: &str) -> User {
    User {
        username: username,
        password: pass,
        active: false
    }
}
