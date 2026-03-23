#![allow(dead_code, unused_variables)]

use std::{fs::File, io::ErrorKind};
use std::io::{self, Read};

fn main() {
    /* UNRECOVERABLE ERROR */
    println!("Hello, world!");

    // panic!("PANICCCCCCCC"); // Panic macro --> raises a panic to stop execution

    // println!("Unreachable code"); // unreachable

    /* RECOVERABLE ERRORS */
    recoverable_errors();
}

fn recoverable_errors() {
    // let r = divide(4, 0); // panicked at divide by 0
    // println!("Res = {r}");

    let r = match recoverable_divide(4, 0) {
        Ok(value) => value,
        Err(err) => {
            println!("Error: {err}");
            -1 // returning -1 by default
        }
    }; // will not panic here
    println!("Res = {r}");

    file_rw();
    let r = read_username_from_file();
    println!("{:?}", r);
}

fn divide(x: i32, y: i32) -> i32 {
    x / y
}

fn recoverable_divide(x: i32, y: i32) -> Result<i32, String> {
    // Handling recoverable errors using Result<T, E> type
    if y == 0 {
        return Err(String::from("Division by 0 not possible")); // returning string on error
    }

    Ok(x / y)
}

fn file_rw() {
    let greeting_file_result = File::open("hello.txt"); // returns Result<File, Error>

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("File not found"), // panics if file not present --> unrecoverable
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                // creating the file if not found
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {error:?}"),
            },
            _ => {
                // all otther errors
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}

// error propagation
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // "?" --> if error encountered return error to the owner --> similar to match
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
