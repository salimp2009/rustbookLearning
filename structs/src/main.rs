#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let mut user1 = User {
        active: true,
        email: String::from("blabla@zort.com"),
        username: String::from("blabla"),
        sign_in_count: 1,
    };
    println!("User info: {:?} ", user1);
    println!("user1 email: {}", user1.email);
    println!("user1 username: {}", user1.username);

    let mut user2 = User {
        active: true,
        email: String::from("blabla@zort.com"),
        username: String::from("demiros"),
        sign_in_count: 1,
    };

    user2.email = String::from("demrios@demiros.com");
    println!("user2: {:?}", user2);

    let user3 = User {
        email: String::from("didolos@homikos.com"),
        // username: String::from("didolos"),
        ..user1
    };
    println!("user3: {:?}", user3);
    assert_eq!(user1.sign_in_count, 1);
    // we can username of user1 since String is moved but other fields are valid
    //    assert_eq!(user1.username, String::from("blabla"));
    user1.username = String::from("yowww_yowww");
    println!("user1 after the move : {:?}", user1);
}
