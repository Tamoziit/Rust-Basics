// src/lib.rs <-- Root library crate
/* Better Organisation & Modularity */
#![allow(dead_code, unused_variables)]

pub mod auth_utils;
mod database; // making database.rs part of the module tree

use auth_utils::{login, models};
use database::{Status, connect_to_db};

pub fn authenticate(cred: models::Credentials) {
    // public func.
    if let Status::Connected = connect_to_db() {
        // calling from database module
        login(cred);
    }
}
