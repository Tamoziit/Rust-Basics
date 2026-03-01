fn main() {
    println!("Hello, world!");

    my_function(90, true);
    expression();

    let x = five();
    println!("{x}");
}

fn my_function(x: i32, y: bool) { // function signature
    println!("x = {x}, y = {y}");
}

fn expression() {
    let y = {
        let x = 10;
        x + 1 // no semi-colon required
    };
    println!("{y}");
}

fn five() -> i32 { // type defn.
    5
}