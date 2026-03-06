#[derive(Debug)] // deriving Debug trait for rectangle
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

pub fn struct_debug_display(rect: &Rectangle) {
    println!("Debug Rectangle = {:?}", rect); // :? --> debugged printing of struct
    println!("Pretty Debug Rectangle = {:#?}", rect); // :#? --> pretty debug
}