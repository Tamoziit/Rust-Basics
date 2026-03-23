use std::collections::HashMap;

pub fn teams() {
    let mut scores: HashMap<String, i32> = HashMap::new(); // key-value pair

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 20);

    scores.entry(String::from("Red")).or_insert(50); // if value of red exists continue, else assign 50 to key "Red"

    println!("Hashmap: {:?}", scores);

    let score_of_blue = scores.get(&String::from("Blue")).unwrap_or(&0); // get() doesn't take ownership, it only takes reference; also it returns Option<T> so we return 0 on unwrapping/default(None)
    println!("Score = {score_of_blue}");

    // iteration
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

pub fn word_count() {
    let text = "Hash maps are useful when you want to look up data not by using an index, as you can with vectors, but by using a key that can be of any type";
    let mut map: HashMap<&str, i32> = HashMap::new();

    for word in text.split_whitespace() {
        // splitting the string at whitespaces
        let count = map.entry(word).or_insert(0); // if the word doesn't exist in hashmap, initialize with 0
        *count += 1; // else increment count
    }

    for (key, value) in &map {
        println!("{key}: {value}");
    }
}
