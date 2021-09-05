struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        
        // make other fields same as user1
        ..user1
    };

    user2.email = String::from("updatedemail@example.com");

    let user3 = build_user(String::from("myemail@example.com"),
     String::from("user3"));

    println!("{}, {}, {}", user2.email, user2.sign_in_count, user3.username);
}

// We can write email and username like this because
// parameter of the functions has same name with
// User struct field names
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
