fn main() {
    // Tuples --> fixed length; heterogenous DType
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{:?}", tup);

    let record = ("Tamojit", 24, true, 56_00_00_000, 893.993874);
    let (v, w, x, y, z) = record;
    println!("{v}, {w}, {x}, {y}, {z}");
    println!("{}, {}, {}", record.0, record.1, record.2);

    // Arrays --> fixed length; homogenous
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // [type; size]
    println!("{:?}", a);

    let b = [10; 5]; // [10, 10, 10, 10, 10]
    println!("{:?}", b);
}
