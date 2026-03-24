#![allow(dead_code, unused_variables, unused_assignments)]

fn main() {
    let x = String::from("dangling"); // lifetime 'b starts
    let y = String::from("reference"); // lifetime 'a starts

    let res = longest(&x, &y); // inside valid lifetime 'a
    // lifetime 'a & 'b ends & lieftime of res is within shortest lifetime 'a
    println!("Longest = {res}");

    let x = String::from("dangling"); // lifetime 'b
    let res: &str; // inside lifetime of 'b

    {
        let y = String::from("reference"); // lifetime 'a
        // res = longest(&x, &y); // not possible here for the lifetime defn.
    } // lifetime 'a ends
    // println!("Longest = {res}"); // res cannot reference 'a after its end while being inside 'b

    /*
       Static Lifetimes:
       The above code runs for &str typed strings even if lifetime of result outlives 'a --> This is because &str typed strings are STATICALLY TYPED --> they are stored in the program binary (build folder) and not in heap memory --> hence they do not create dangling references
    */
    let x = "dangling"; // lifetime 'b
    let res: &str; // inside lifetime of 'b

    {
        let y = "reference"; // lifetime 'a
        res = longest(x, y);
    } // lifetime 'a ends
    println!("Longest = {res}");
}

fn dangling_reference() {
    let result: &i32;

    {
        let x = 5;
        result = &x;
    } // lifetime of x ended here

    // println("{result}"); // cannot print since x moved out of scope & result is still holding the reference to x --> dangling reference
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // 'a --> lifetime defn. --> it states the Rust Borrow Checker that the shortest lifetime b/w x & y (here 'a) should be valid uptil the lifetime of the result returned --> without this the function may return a reference to an out of scope str
    if x.len() > y.len() {
        return x;
    }

    return y;
}
