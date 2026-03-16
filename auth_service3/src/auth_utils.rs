pub mod models;

pub fn login(cred: models::Credentials) {
    // models inside its own module
    // try to login user
    super::database::get_user(); // going one step up from curr.
    // crate::database::get_user(); // another way --> from root crate --> database module --> get_user()
}
