/*
When we follow the common rules of ownership --> to calculate the length of the string & for it to not get out of scope after calculating we need to -->
- transfer ownership to length function
- calculate length
- return the string & length to take back the ownership
*/
pub fn ownership_fiasco(s: String) -> (String, usize) {
    let l = s.len(); // ownership taken
    (s, l) // ownership given back
}

/*
Reference:
Not taking ownership --> but reference (pointer) to the object in heap --> BORROWING
*/
pub fn reference_fn(s: &String) -> usize {
    // s.push_str(" hello"); // not possible even if the owner is mutable because --> we cannot change the value of the owner from an immutable reference
    let res = s.len();
    res
}

pub fn mutable_ref(s: &mut String) -> usize {
    // passing mutable reference
    s.push_str(" Hello");
    s.len()
}

pub fn _multi_mutable_ref() {
    let mut s = String::from("hello");

    let _r1 = &mut s;
    // let r2 = &mut s; // we cannot pass mutable references to more than 1 owner, at the same scope;
    // let r3 = &s; // even passing immutable ref is not possible after passing mutable reference, at the same scope;

    // println!("{r1}, {r2}, {r3}");

    /*
    Different scope mutability
    */
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        r1.push_str(" wolrd!!");

        println!("{}", r1.len());
    }

    let r2 = &mut s; // scope of r1 ended outside its scope, hence a new mutable reference can be made
    r2.push_str(" DHU RAN DHARRRRRRRR");

    println!("{}", r2.len());

    let mut s2 = String::from("Hmmmm");

    let r1 = &s2; // immutable
    let r2 = &s2; // immutable
    println!("{r1}, {r2}"); // r1, r2 not used anymore --> end of scope

    let r3 = &mut s2; // here mutable ref allowed because no active mutable or immutable ref. available
    r3.push_str("foo");
    println!("{r3}");
}
