#[derive(Debug)]
pub enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

pub fn execute() {
    let cells: Vec<SpreadSheetCell> = vec![
        SpreadSheetCell::Int(30),
        SpreadSheetCell::Text(String::from("Hello")),
        SpreadSheetCell::Float(2.34),
        SpreadSheetCell::Text(String::from("Hi")),
    ]; // Heterogenous type vector

    println!("My vector = {:#?}", cells);

    match cells.get(1) {
        Some(SpreadSheetCell::Int(value)) => println!("int value = {value}"),
        Some(value) => println!("Not integer {:?}", value),
        None => println!("No value"),
    };
}
