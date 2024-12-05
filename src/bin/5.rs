use std::{cmp::Ordering, collections::HashMap, env};

use ad24::Input;

fn main() {
    let input = Input::init(5, env::args().nth(1).unwrap()).unwrap();
    let (orders, updates) = get_parsed_input(input.content);

    let mut reverse_map: HashMap<i32, Vec<i32>> = HashMap::new();

    orders.clone().iter().for_each(|order| {
        if let Some(before_pages) = reverse_map.get(&order[1]) {
            let mut o_clone = before_pages.clone();
            o_clone.push(order[0]);
            reverse_map.insert(order[1], o_clone);
        } else {
            reverse_map.insert(order[1], vec![order[0]]);
        }
    });

    let mut middle_sum = 0;
    let mut fixed_middle_sum = 0;

    updates.iter().for_each(|update| {
        if validate_line(update, reverse_map.clone()) {
            middle_sum += update[update.len() / 2];
        } else {
            let mut update_mut = update.clone();
            update_mut.sort_by(|a, b| should_swap(reverse_map.clone(), a, b));
            fixed_middle_sum += update_mut[update_mut.len() / 2];
        }
    });

    println!("{{\"day\": 5, \"part1\": {middle_sum}, \"part2\": {fixed_middle_sum}}}");
}

fn validate_line(update: &[i32], reverse: HashMap<i32, Vec<i32>>) -> bool {
    for i in 0..update.len() {
        let succeeding = &update[i + 1..update.len()];
        let mut result = true;

        succeeding.iter().for_each(|succeeding| {
            if let Some(not_allowed) = reverse.get(&update[i]) {
                if not_allowed.contains(succeeding) {
                    result = false;
                }
            }
        });
        if !result {
            return false;
        }
    }

    true
}

fn should_swap(reverse: HashMap<i32, Vec<i32>>, a: &i32, b: &i32) -> Ordering {
    if let Some(not_allowed) = reverse.get(a) {
        if not_allowed.contains(b) {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

fn get_parsed_input(input: String) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let lines: Vec<String> = input.lines().map(|line| line.to_owned()).collect();

    let inp_split: Vec<_> = lines.split(|val| val.is_empty()).collect();

    assert_eq!(inp_split.len(), 2);

    let orders: Vec<_> = inp_split[0]
        .iter()
        .map(|order| {
            order
                .split("|")
                .map(|page| page.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();

    let updates: Vec<_> = inp_split[1]
        .iter()
        .map(|update| {
            update
                .split(",")
                .map(|page| page.trim().parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    (orders, updates)
}
