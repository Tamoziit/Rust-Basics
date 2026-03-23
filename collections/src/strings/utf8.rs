pub fn utf8_strings() {
    println!("{}", String::from("السلام عليكم"));
    println!("{}", String::from("Dobrý den"));
    println!("{}", String::from("Hello"));
    println!("{}", String::from("שלום"));
    println!("{}", String::from("नमस्ते"));
    println!("{}", String::from("こんにちは"));
    println!("{}", String::from("안녕하세요"));
    println!("{}", String::from("你好"));
    println!("{}", String::from("Olá"));
    println!("{}", String::from("Здравствуйте"));
    println!("{}", String::from("Hola"));

    let mut hello = String::from("नमस्ते");

    // mutating strings
    hello.push_str(" in hindi"); // pushing string
    hello.push('.'); // pushing character

    println!("{hello}");
}
