#[derive(Debug)]
enum _Message {
    // better than building separate structs for each data in enum
    Quit, // no data
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

pub fn option_enum() {
    println!("{:?}", Option::Some("1"));
    println!("{:?}", Option::<&str>::None); // since none has no value --> we implicitly specify its type

    let op: Option<i32> = Some(1);
    let op2: Option<i32> = None; // Hits PANIC on normal unwrap
    let x = 2;

    let sum = x + op.unwrap(); // value unwrapping
    println!("Sum = {sum}");
    let sum = x + op2.unwrap_or(0); // if no value take default 0 instead of hitting PANIC
    println!("Sum = {sum}");
}
