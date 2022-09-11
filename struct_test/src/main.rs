fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        email: String::from("alww@kth.se"),
        username: String::from("alww"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    let user2 = build_user(String::from("kth.se"), String::from("kth"));
    let user3 = User {
        email: String::from("al.se"),
        ..user2
    };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u16,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
