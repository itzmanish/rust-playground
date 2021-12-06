// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added,
// so “first” becomes “irst-fay.” Words that start with a vowel have “hay”
// added to the end instead (“apple” becomes “apple-hay”).
// Keep in mind the details about UTF-8 encoding!

fn convert_pig_latin(string: &str) -> String {
    let mut result = String::new();
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let first = string
        .chars()
        .next()
        .expect("Given string should not be empty");
    if vowels.contains(&first) {
        result.push_str(string);
        result.push_str("-hay")
    } else {
        result = string[1..].to_string();
        let formatted = format!("-{}ay", first);
        result.push_str(&formatted[..]);
    }
    result
}

fn main() {
    println!("first: {}", convert_pig_latin("first"));
    println!("apple: {}", convert_pig_latin("apple"));
    println!("noah: {}", convert_pig_latin("noah"));
    println!("amazon: {}", convert_pig_latin("amazon"));
}
