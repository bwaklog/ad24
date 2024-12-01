use ad24::Input;
use std::{collections::HashMap, fs};

fn main() {
    let input_data = Input::init(1, std::env::args().nth(1).unwrap()).unwrap();

    let content = fs::read_to_string(input_data.dataset).expect("failed to read contents");

    let mut left: Vec<i32> = Vec::with_capacity(1000);
    let mut right: Vec<i32> = Vec::with_capacity(1000);

    for line in content.lines() {
        if line.len() != 0 {
            let mut split_lines = line.split("   ").map(|x| x.trim().parse().unwrap());
            left.push(split_lines.next().unwrap());
            right.push(split_lines.next().unwrap());
        }
    }

    left.sort();
    right.sort();

    assert_eq!(left.len(), right.len());

    let diff: i32 = right
        .iter()
        .zip(left.iter())
        .map(|(x, y)| (x.to_owned() - y.to_owned()).abs())
        .sum();

    let mut right_hash: HashMap<i32, i32> = HashMap::with_capacity(1000);
    for val in right.clone() {
        if let Some(x) = right_hash.get(&val) {
            right_hash.insert(val, x + 1);
        } else {
            right_hash.insert(val, 1);
        }
    }

    let similarity_sum: i32 = left
        .iter()
        .map(|x| x * right_hash.get(x).unwrap_or(&0))
        .sum();

    println!("{{\"diff\":{diff},\"similarity_sum\": {similarity_sum}}}");
}
