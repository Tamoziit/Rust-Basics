pub fn if_let() {
    // without if let
    let config_max: Option<u8> = Some(3_u8);
    match config_max {
        Some(max) => println!("Without if let: The max is configured to be {max}"),
        _ => ()
    }

    // with if let
    if let Some(max) = config_max  {
        println!("With if let: The max is configured to be {max}");
    } // here we do not need to handle _/None condition
}