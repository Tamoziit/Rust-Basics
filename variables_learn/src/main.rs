fn main() {
    let age = 21; // Int32 by default
    //age = 25; // cannot be changed --> Rust by default has immutable variables
    println!("Age = {age}");

    let mut age2 = 25; // mutable variable
    println!("Mutable Age = {age2}");
    age2 = 30; // no error here
    println!("Mutable Age = {age2}");

    println!("Age 1 = {age}, Age 2 = {}", age2 + 10);
}
