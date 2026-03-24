mod struct_generics;

fn main() {
    let list = vec![1, 2, 3, 4, 10, 6];
    let l = largest(&list);
    println!("largest = {l}");

    let list = vec![9.8, 7.34, 0.67, 2.0];
    let l = largest(&list); // automatically type <T> is infered as f64
    println!("largest = {l}");

    let list = vec!['a', '2', '3', 'f'];
    let l = largest(&list);
    println!("largest = {l}");

    struct_generics::create_points();
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    // T --> any type which is automatically infered during runtime --> Generics
    // The Partial Order(PartialOrd) Trait defines that, generic T will be infered for types which only has partial order trait (comparisons permitted, ie, we cannot compare 2 structs or enums as they do not implement partial ordering)
    let mut result = &list[0];

    for item in list {
        if item > result {
            result = item;
        }
    }

    result
}
