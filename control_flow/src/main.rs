fn main() {
    // infinitely running loop
    // loop {
    //     println!("Hello, world!");
    // }

    let mut num = 0;
    loop {
        println!("num = {num}");

        if num == 10 {
            break;
        }
        num = num + 1;
    }

    // loops are expressions returning a result
    let mut num = 0;
    let result = loop {
        println!("num = {num}");

        if num == 10 {
            break 70;
        }
        num = num + 1;
    };

    println!("Result = {result}"); // value of result returned

    loop_label();
    while_loop();
    for_loop();
}

fn loop_label() {
    let mut num = 0;
    let result = 'my_loop: loop {
        if num == 10 {
            break 70;
        }

        loop {
            if num == 20 {
                break 'my_loop 40;
            }

            num += 2;
        }
    };
    println!("result = {result}");
}

fn while_loop() {
    let arr = [1, 2, 3, 4, 5];
    let mut index = 0;

    while index < 5 {
        println!("{}", arr[index]);
        index = index + 1;
    }
}

fn for_loop() {
    let arr = [1, 2, 3, 4, 5];

    for element in arr {
        println!("{}", element);
    }

    for x in 1..=10 { // inclusive range
        println!("{x}");
    }
    for x in (1..=10).rev() { // reversed inclusive range
        println!("{x}");
    }
}