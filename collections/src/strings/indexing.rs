use unicode_segmentation::UnicodeSegmentation;

pub fn indexing() {
    let hello = String::from("नमस्ते");

    // let first_char = &hello[0]; // For UTF8 encoded string --> reference by index is not possible as in ASCII

    // "नमस्ते" in bytes --> [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    for c in hello.as_bytes() {
        println!("c = {c}");
    }

    // "नमस्ते" in scalar --> ['न', 'म', 'स', '्', 'त', 'े'] (with diatrics --> matra)
    for c in hello.chars() {
        println!("c = {c}");
    }

    // "नमस्ते" in Grapheme Clusters --> ["न", "म", "स्", "ते"]
    // to print grapheme clusters --> we need an external crate
    for e in hello.graphemes(false).collect::<Vec<&str>>() { // collecting grapheme clusters in a string vector
        println!("element = {e}");
    }
}
