#[derive(Debug)]
#[allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        email: String::from("blabla@zort.com"),
        username: String::from("blabla"),
        sign_in_count: 1,
    };
    println!("User info: {:?} ", user1);
}
