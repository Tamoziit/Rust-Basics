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
