// Hashmap and operation on them

fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    // keys are unique and gets override with latest value if already
    // exists in hashmap
    scores.insert(String::from("Blue"), 32);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
