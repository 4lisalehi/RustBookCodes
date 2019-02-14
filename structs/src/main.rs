struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Point(i32, i32, i32);
struct Color(i32, i32, i32);


fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 1,
    }
}


fn main() {
    let mut user1 = User {
        email: String::from("alisalehi1995@gmail.com"),
        username: String::from("alisalehi"),
        sign_in_count: 0,
        active: true,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someoneusername"),
        ..user1
    };
    user2.email = String::from("anotherone@example.com");
    user1.username = String::from("Alis");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
