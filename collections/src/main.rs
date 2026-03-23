mod enum_vec;
use enum_vec::execute;
use collections::strings::{utf8, indexing};

fn main() {
    let mut vec = Vec::new(); // vector: homogenous collection

    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("vector = {:?}", vec);

    // macro-generics
    let mut vec = vec![5, 6, 7];
    vec.push(8);

    println!("vector = {:?}", vec);

    // immutable vectors
    let vec = vec![5, 6];
    // vec.push(1); // NOT POSSIBLE
    println!("vector = {:?}", vec);

    let vec: Vec<i32> = vec![]; // empty immutable vector (Type annotation is necessary here)
    println!("vector = {:?}", vec);

    // Reading vectors

    /* Allowing Panic on Index out of Bound */
    let v1 = vec![1, 2, 3, 4, 5];
    let fourth_val = &v1[3]; // taking reference, not ownership --> causes panic for invalid idx
    println!("{fourth_val}");

    /* NOT allowing panic on index ouut of bound */
    let fourth_val = v1.get(3); // returns an option hence never panics
    match fourth_val {
        Some(fourth_val) => println!("{fourth_val}"),
        None => println!("There is no 4th element."),
    }

    // Iterating
    let mut v1 = vec![1, 2, 3, 4];

    for i in &mut v1 {
        // taking mutable ref of v1 to mutate its val
        println!("val = {i}");
        *i = *i * 2; // deferencing the value
    }

    println!("{:?}", v1);

    execute();
    utf8::utf8_strings();
    indexing::indexing();
}
