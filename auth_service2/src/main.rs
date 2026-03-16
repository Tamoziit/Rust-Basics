// src/main.rs <-- Root Binary Crate, i.e., execution starts from here
use auth_service2::auth_utils::models::Credentials;
use auth_service2::authenticate; // getting lib crates into scope

fn main() {
    let cred: Credentials = Credentials {
        username: String::from("Tamojit"),
        password: String::from("abc123#"),
    };

    authenticate(cred); // calling authenticate from lib
}
