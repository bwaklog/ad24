use std::env;

use ad24::Input;
use regex::Regex;

fn main() {
    let input = Input::init(3, env::args().nth(1).unwrap()).unwrap();

    let mul_re = Regex::new(r"mul\((?<num1>\d*),(?<num2>\d*)\)").unwrap();
    let triggers_re =
        Regex::new(r"((?<mul>mul\(\d*,\d*\))|(?<dont>don\'t\(\))|(?<do>do\(\)))").unwrap();

    let result = allow_muls(mul_re, input.content.trim());

    let result2 = dont_allow_muls(triggers_re, input.content.trim());

    println!("{{\"day\": 3, \"part 1\": {result}, \"part 2\": {result2}}}")
}

fn allow_muls(re: Regex, hay: &str) -> i32 {
    re.captures_iter(hay)
        .map(|c| c.extract())
        .map(|(_, [num1, num2])| num1.parse::<i32>().unwrap() * num2.parse::<i32>().unwrap())
        .sum()
}

fn dont_allow_muls(re: Regex, hay: &str) -> i32 {
    let mul_re = Regex::new(r"mul\((?<num1>\d*),(?<num2>\d*)\)").unwrap();
    let mut allow = true;
    let mut total = 0;

    re.captures_iter(hay)
        .map(|c| c.extract())
        .for_each(|(_, [cap, _])| {
            if cap.contains("do()") {
                allow = true;
            } else if cap.contains("don't()") {
                allow = false;
            } else if cap.contains("mul") && allow {
                let Some(mul) = mul_re.captures(cap) else {
                    return;
                };
                total += mul["num1"].parse::<i32>().unwrap() * mul["num2"].parse::<i32>().unwrap();
            }
        });

    total
}
