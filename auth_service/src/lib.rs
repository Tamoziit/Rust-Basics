// src/lib.rs <-- Root library crate
#![allow(dead_code, unused_variables)]

pub struct Credentials {
    pub username: String, // owned typed
    pub password: String, // in Rust --> by default, fields are also private
}

enum Status {
    // DB connection status
    Connected,
    Interrupted,
}

fn connect_to_db() -> Status {
    // assuming connected
    Status::Connected
}

fn get_user() {
    // fetch user from DB
}

fn login(cred: Credentials) {
    // try to login user
    get_user();
}

pub fn authenticate(cred: Credentials) {
    // public func.
    if let Status::Connected = connect_to_db() {
        // AUTHENTICATE
    }
}
