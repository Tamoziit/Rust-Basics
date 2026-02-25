use rand::Rng;
use std::cmp::Ordering;
use std::io; // standard I/O lib

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // = --> inclusive limits

    loop {
        // loop for retrying
        println!("Enter a guess b/w 1-100:");

        let mut guess = String::new(); // mutable string initialization
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input"); // passing mutable reference to read_line, so that it can mutate its value --> .expect() --> optimistically handling errors, where on encountering an error/exception we print the mentioned string

        let guess_number: u32 = match guess.trim().parse() {
            Ok(num) => num, // if number parsing successfull --> return number
            Err(err) => {
                println!("Please enter a valid input: {}", err);
                continue;
            }
        }; // parsing string into unsigned int 32; trim() is to omit whitespaces, \n, \t etc. from user i/p.

        println!("You guessed {guess_number}");
        println!("The secret number is: {secret_number}");

        match guess_number.cmp(&secret_number) {
            // passing reference of secret number to match --> analogous to switch case
            Ordering::Equal => {
                println!("You Won");
                break;
            }
            Ordering::Greater => println!("Too Big"),
            Ordering::Less => println!("Too Small"),
        }; // match Ordering is exhaustive --> all its cases must be handled
    }
}
