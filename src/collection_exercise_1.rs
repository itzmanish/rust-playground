// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn main() {
    let mut list_int: Vec<i32> = vec![1, 2, 5, 2, 2, 6, 7, 3, 4, 6, 23, 56, 33, 37, 45, 9, 94, 70];
    println!(
        "mean: {}, median: {}, mode: {}",
        mean(&list_int),
        median(&mut list_int),
        mode(&list_int)
    );
}

fn mean(input: &Vec<i32>) -> f32 {
    input.iter().sum::<i32>() as f32 / input.len() as f32
}

fn median(input: &mut Vec<i32>) -> i32 {
    input.sort();
    input[input.len() / 2]
}

fn mode(input: &Vec<i32>) -> i32 {
    let mut map: HashMap<i32, u8> = HashMap::new();
    let mut max_key: i32 = 0;
    let mut max_value: u8 = 0;
    for v in input.iter() {
        let count = map.entry(*v).or_insert(0);
        *count += 1;
    }
    for (key, value) in map.iter() {
        if max_value < *value {
            max_value = *value;
            max_key = *key;
        }
    }
    max_key
}
