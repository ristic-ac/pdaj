use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    
    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("After overwrite: {:?}", scores);

    // Adding a key-value pair only if the key isn’t present
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("After conditional insert: {:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut word_counts = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_counts.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word counts: {:?}", word_counts);
}
