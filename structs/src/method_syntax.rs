#[derive(Debug)]
pub struct Rectangle2 {
    pub width: u32,
    pub height: u32,
}

impl Rectangle2 {
    // method implementation (like class object)
    pub fn calculate_area(&self) -> u32 {
        self.height * self.width
    }

    pub fn can_hold(&self, other: &Rectangle2) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    pub fn square(side: u32) -> Rectangle2 { // fn directly available over Rectangle2 without an object because its 1st parameter isn't self
        Rectangle2 {
            width: side,
            height: side,
        }
    }
}
