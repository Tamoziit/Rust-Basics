fn main() {
    let mut s = String::from("Hello World");
    let res = find_first_word(&s); // passing reference

    s.clear(); // for this state after clearing the string, result should ideally change --> but it does because, scope of input/reference is no longer active & res has already been computed

    println!("1st space of {s} at {res}");

    /*
    Slice type
     */
    let mut s = String::from("Hello World");
    let hello = &s[0..5]; // from 0th idx to 4th idx
    let world = &s[6..11];

    // s.clear(); // NOT VALID --> Slice holds the reference to [m..n] of s --> the reference is still in scope & hence the parent cannot be dropped
    // s.push_str("hellyeah"); // same here

    println!("{hello}, {world}");
    s.clear(); // VALID --> after slice refs have moved out of scope

    // slicing soln.
    let mut s = String::from("Hello World");
    let res = slicing_soln(&s);

    println!("1st space of {s} at {}", res.len());
    s.clear(); // can be cleared after slice references move out of scope
}

fn find_first_word(input: &String) -> usize {
    let s = input.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        // looping through each byte/character of string
        if item == b' ' {
            // btye-space
            return i;
        }
    }

    s.len()
}

fn slicing_soln(input: &str) -> &str {
    let s = input.as_bytes();

    for (i, &item) in s.iter().enumerate() {
        if item == b' ' {
            return &input[..i]; // from 0 to i-1
        }
    }

    return &input[..]; // from 0 to s.len() - 1
}
