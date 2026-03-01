/*
MEMORY SAFETY
*/
mod ownership_fn;
mod references;

fn main() {
    /*
       Statically typed string:
       - Allocated in Stack Memory [size known at compile time]
       - Not growable (static memory)
    */
    let s = "Hello";
    println!("{s}");

    /*
       Dynamic String:
       - Allocated in Heap Memory [size unknown at compile time]
       - Growable (dynamic memory)
    */
    let mut s = String::from("Hello");
    println!("{s}");

    s.push_str(" World");
    println!("{s}");

    double_free();
    stack_copy();

    let s = String::from("Ownership");
    ownership_fn::takes_ownership(s);

    // println!("s = {s}"); // not possible since again due to move in heap owner of s is takes_ownership() & s no longer exists in scope

    let s = ownership_fn::gives_ownership();
    println!("s = {s}"); // now s is the owner of the string returned by gives_ownership()

    let s2 = ownership_fn::takes_and_gives_back(s); // s gives ownership to takes_and_gives_back() --> s moves out of scope --> this func then returns & transfers ownership to s2 & moves out of scope
    println!("ownership given s2 = {s2}");

    // Normal ownership transfer length calc.
    let st = String::from("HellYeaaaaaaaah");
    let (st, l) = references::ownership_fiasco(st); // ownership given & taken back [throught the tuple return]

    println!("Length of {st} = {l}");
}

/*
By the concept of Ownership:
- Value in Heap (here dynamic String --> s) is allocated by Rust
- The memory size is dynamically altered by us (through push_str(), etc.)
- Once 's' moves out of scope --> after end of main() --> memory held by s in heap is automatically freed (Garbage collection).
*/

fn double_free() {
    let s1 = String::from("Hello");

    let s2 = s1; // s2 points to the same memory in Heap

    // println!("s1 = {s1}"); // This throws an error -->
    /*
    Since s1 & s2 points to the same memory in heap -->
    When s1 moves out of scope --> s2 is dangling
    When s2 moves out of scope --> it tries to "Double Free" a heap object

    In order to prevent this --> on s2 = s1 --> s1 instantly moves out of scope and only s2 points to the memory in heap

    So, we don't actually Deep copy or shallow copy --> We "move" the data
    */

    println!("s2 = {s2}");

    // To actually make a copy
    let s1 = String::from("Hello 2");

    let mut s2 = s1.clone(); // Deep copy [different pointers in heap]
    s2.push_str(" cloned");

    println!("s1 = {s1}");
    println!("s2 = {s2}");
}

fn stack_copy() {
    let x = 5;
    let y = x; // copy of static types on stack

    println!("x = {x}, y = {y}");
}
