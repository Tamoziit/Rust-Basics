fn main() {
    let age = 21; // Int32 by default
    //age = 25; // cannot be changed --> Rust by default has immutable variables
    println!("Age = {age}");

    let mut age2 = 25; // mutable variable
    println!("Mutable Age = {age2}");
    age2 = 30; // no error here
    println!("Mutable Age = {age2}");

    println!("Age 1 = {age}, Age 2 = {}", age2 + 10);

    const PI: u8 = 3; // always immutable & should be explicitly assigned type
    println!("PI = {PI}");

    // const SECONDS: u32 = 60 * 60 * 3 + age; // not valid since age is resolved in run-time & constants cannot be rsolved in run-time

    let apples = 8;
    println!("1st value = {apples}");

    let apples = 100; // shadowed value (not mutating, but re-defining)
    println!("1st shadowed value = {apples}");

    let apples = true; // in shadowing, we can  change the type as well, not possible but just "mut"
    println!("2nd shadowed value = {apples}");

    test();
    safety();
}

fn test() {
    let x = 5;

    let x = x + 1; //shadowed --> 5 + 1 = 6

    {
        let x = x * 2;
        println!("inner scope x = {x}"); // shadowed x = 6 * 2 = 12 --> NB, this x is valid inside this scope only, outside this scope,  is still 6
    }

    println!("Outer scope x = {x}")
}

fn safety() {
    // initially the value is mutable to set log in state
    let mut user_is_logged_in: bool = false;
    user_is_logged_in = true;

    if user_is_logged_in {
        // here, we shadow the boolean variable into string, just to store log in state message immutable for the rest of the code, so that the state stays consistent throughout the system.
        let user_is_logged_in = "User is logged in";
        println!("{user_is_logged_in}");

        // user_is_logged_in = "NO"; // not possible because of immutability 
    } else {
        let user_is_logged_in = "User is NOT logged in";
        println!("{user_is_logged_in}");
    }
}
