use std::io; // standard I/O lib

fn main() {
    println!("Guess the number!");

    println!("Enter a guess b/w 1-100:");

    let mut guess = String::new(); // mutable string initialization
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input"); // passing mutable reference to read_line, so that it can mutate its value --> .expect() --> optimistically handling errors, where on encountering an error/exception we print the mentioned string

    println!("You guessed {guess}");
    // println!("You guessed {}", guess);
}
