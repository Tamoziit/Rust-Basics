pub fn takes_ownership(s: String) {
    println!("Inside takes ownership {s}");
}

pub fn gives_ownership() -> String {
    let s = String::from("Ownership given");
    s
}

pub fn takes_and_gives_back(s: String) -> String {
    println!("Ownership taken {s}"); // ownership taken
    s //ownership given
}