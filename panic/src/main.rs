fn main() {
    /*
       Range of u8 = 0 to 255
       a = 200 + 100 = 300 -->OVERFLOW
       --> but 200 is only resolved in runtime --> compiler doesn't know about Overflow before runtime execution --> so it gives PANIC ERROR (in debug mode)
       -->  compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping.
    */

    //let a: u8 = 200 + 100; // resolved error at compile-time

    // let a: u8 = panic_func() + 100; // not resolved before runtime
    // println!("{a}");

    // ways to tackle overflow by custom wrapping
    let a: u8 = panic_func().wrapping_add(100);
    println!("Wrapping add: {a}"); // 2's comp wrapped addition

    let a = match panic_func().checked_add(57) {
        Some(num) => num,
        None => {
            println!("Cannot add"); // cannot add on overflow
            return;
        }
    };
    println!("Checked add: {a}");
}

fn panic_func() -> u8 {
    200 // returns 200
}
