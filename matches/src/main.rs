mod if_let;

#[derive(Debug)]
enum USStates {
    Alabama,
    LA,
    California,
    LasVegas,
}

#[derive(Debug)]
enum Coin {
    Peny,
    Nickel,
    Dime,
    Quarter(USStates),
}

fn main() {
    let coin = Coin::Peny;
    println!("Value = {}", value_in_cent(&coin));

    let coin = Coin::Quarter(USStates::California);
    println!("Value = {}", value_in_cent(&coin));

    let coin = Coin::Quarter(USStates::Alabama);
    println!("Value = {}", value_in_cent(&coin));

    // Option<T> match
    println!("Sum = {}", add(50, Some(90)));
    println!("Sum = {}", add(50, None));

    // Dice problem
    let dice_roll = 3;
    dice_roll_game(dice_roll);

    let dice_roll = 7;
    dice_roll_game(dice_roll);

    let dice_roll = 9;
    dice_roll_game(dice_roll);

    if_let::if_let();
}

fn value_in_cent(coin: &Coin) -> u8 {
    match coin {
        // match --> switch case
        Coin::Peny => 1, // if penny --> returning 1
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(USStates::California) => {
            println!("Hello from California");
            25
        }
        Coin::Quarter(state) => {
            // any other state
            println!("Quarter of {:?}", state);
            25
        }
    }
}

fn add(num: i32, num2: Option<i32>) -> i32 {
    match num2 {
        Some(i) => num + i,
        None => num,
    }
}

fn dice_roll_game(dice: i32) {
    match dice {
        3 => println!("Fancy Hat"),
        7 => println!("No Fancy Hat"),
        other => println!("Move {} steps", other), // handling all other cases
        // _ => () // Unit Value --> nothing to execute for other cases (); neither do we need to use other variable (_)
    }
}
