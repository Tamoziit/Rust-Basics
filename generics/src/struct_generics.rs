#[derive(Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
pub struct Point2<T, U> {
    x: T,
    y: U, // now distinct types possible
}

impl<T, U> Point2<T, U> {
    fn new(x: T, y: U) -> Self {
        Self { x, y }
    }

    fn mixup<X, Y>(self, point: Point2<X, Y>) -> Point2<T, Y> {
        Point2 {
            x: self.x,
            y: point.y,
        }
    }
}

pub fn create_points() {
    let a = Point { x: 5, y: 7 };
    let b = Point { x: 10.4, y: 6.8 };
    // let c = Point { x: 10.4, y: 6 }; // Not possible as according to defn., both the parameters are of same type T

    println!("{:?}\n{:?}", a, b);

    // Distinct types
    let a = Point2 { x: 5, y: 'a' };
    let b = Point2 { x: 10, y: 6.8 };
    let c = Point2 { x: 10.4, y: 6 };

    println!("{:?}\n{:?}\n{:?}", a, b, c);

    let x = Point2::new(4, 5.2);
    let y = Point2::new("hailmary", 3.90);

    println!("{:?}\n{:?}", x, y);

    let z = x.mixup(y);
    println!("z = {:?}", z);
}
