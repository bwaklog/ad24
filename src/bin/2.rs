use std::env;

use ad24::Input;

fn main() {
    let input = Input::init(2, env::args().nth(1).unwrap()).unwrap();

    let lines = input
        .content
        .lines()
        .map(|x| {
            x.split(' ')
                .map(|x| x.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let count = lines
        .clone()
        .into_iter()
        .filter(|record| check_gradient(record.to_owned()))
        .collect::<Vec<Vec<i32>>>()
        .len();

    println!();

    let unsafe_count = lines
        .clone()
        .into_iter()
        .filter(|rec| check_gradient_unsafe(rec.to_owned()))
        .collect::<Vec<Vec<i32>>>()
        .len();

    println!("{{\"day\": 2, \"message\": \"hohoho 🎅\", \"part 1\": {count}, \"part 2\": {unsafe_count}}}");
}

fn is_decreasing(record: &[i32]) -> bool {
    let mut net: f32 = 0.0;
    for i in 1..record.len() {
        if record[i] - record[i - 1] > 0 {
            net += 1.0;
        } else {
            net -= 1.0;
        }
    }
    (net / record.len() as f32) < 0.0
}

fn check_gradient(record: Vec<i32>) -> bool {
    let decreasing = is_decreasing(&record);
    for i in 1..record.len() {
        let diff = record[i] - record[i - 1];
        if (diff < 0) ^ decreasing || !(1..=3).contains(&diff.abs()) {
            // println!("unsafe {:?}", record);
            return false;
        }
    }

    // println!("safe {:?}", record);
    true
}

fn check_gradient_unsafe(record: Vec<i32>) -> bool {
    let decreasing = is_decreasing(&record);

    for i in 1..record.len() {
        let diff = record[i] - record[i - 1];
        if (diff < 0) ^ decreasing || !(1..=3).contains(&diff.abs()) {
            // print!("{:?} | removing {} => ", record, record[i - 1]);
            // stdout().flush().unwrap();

            let mut dup1 = record.clone();
            let mut dup2 = record.clone();

            dup1.remove(i);
            dup2.remove(i - 1);

            return check_gradient(dup1) || check_gradient(dup2);
        }
    }

    // println!("safe {:?}", record);
    true
}
