// src/lib.rs <-- Root library crate
/* Better Organisation */
#![allow(dead_code, unused_variables)]

mod database {
    // Database Module
    pub enum Status {
        // DB connection status
        Connected,
        Interrupted,
    }

    pub fn connect_to_db() -> Status {
        // assuming connected
        Status::Connected
    }

    pub fn get_user() {
        // fetch user from DB
    }
}

pub mod auth_utils {
    pub fn login(cred: models::Credentials) { // models inside its own module
        // try to login user
        super::database::get_user(); // going one step up from curr.
        // crate::database::get_user(); // another way --> from root crate --> database module --> get_user()
    }

    pub mod models {
        // module inside another module
        pub struct Credentials {
            pub username: String,
            pub password: String,
        }
    }
}

pub fn authenticate(cred: auth_utils::models::Credentials) {
    // public func.
    if let database::Status::Connected = database::connect_to_db() { // calling from database module
        auth_utils::login(cred);
    }
}
