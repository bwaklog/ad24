use std::env;

use ad24::Input;

fn main() {
    let input = Input::init(7, env::args().nth(1).unwrap()).unwrap();

    let problems = parse_lines(&input.content);

    let total = eval(&problems, 2);
    let total_concat = eval(&problems, 3);

    println!(
        "{{\"day\": 7, \"total\": {}, \"total_concat\": {}}}",
        total, total_concat
    );
}

fn generate_sequences(vec_len: usize, base: u32) -> Vec<Vec<u32>> {
    (0..u64::pow(base as u64, vec_len as u32))
        .map(|i| {
            (0..vec_len)
                .map(|j| (i as u32) / u32::pow(base, j as u32) % base)
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

#[cfg(test)]
mod tests {
    use crate::generate_sequences;

    #[test]
    fn test_generate_ops() {
        let seq1 = generate_sequences(2, 3);
        assert_eq!(seq1.len(), u64::pow(3, 2) as usize);
        eprintln!("{:?}", seq1);

        let seq2 = generate_sequences(3, 3);
        assert_eq!(seq2.len(), u64::pow(3, 3) as usize);
        eprintln!("{:?}", seq2);
    }
}

fn eval_result(op: &u32, l: &u64, r: &u64) -> u64 {
    if op == &1 {
        (l + r).to_owned()
    } else if op == &0 {
        (l * r).to_owned()
    } else {
        format!("{}{}", l, r).parse::<u64>().unwrap()
    }
}

fn eval(problems: &Vec<(u64, Vec<u64>)>, base: u32) -> u64 {
    let mut total = 0;
    let mut completed = 0;
    problems.iter().for_each(|problem| {
        let operator_comb = generate_sequences(problem.1.len() - 1, base);
        for combination in operator_comb {
            let problem_rhs = problem.1.clone();
            let mut rhs = problem_rhs.iter();
            let mut ops = combination.iter();
            let mut temp: Option<u64> = None;

            loop {
                if let Some(op) = ops.next() {
                    if let Some(val) = temp {
                        temp = Some(eval_result(op, &val, rhs.next().unwrap()));
                    } else {
                        temp = Some(eval_result(op, rhs.next().unwrap(), rhs.next().unwrap()));
                    }
                } else {
                    break;
                }
            }
            if let Some(result) = temp {
                if result == problem.0 {
                    // eprintln!("solved {:?} with operator sequence {:?}", problem, combination);
                    total += problem.0;
                    completed += 1;
                    break;
                }
            }
        }
    });

    total
}

fn parse_lines(input: &String) -> Vec<(u64, Vec<u64>)> {
    let problems: Vec<(u64, Vec<u64>)> = input
        .trim()
        .lines()
        .map(|problem| {
            let mut equation = problem.split(": ");
            (
                equation.next().unwrap().parse::<u64>().unwrap(),
                equation
                    .next()
                    .unwrap()
                    .split(" ")
                    .map(|val| val.parse::<u64>().unwrap())
                    .collect(),
            )
        })
        .collect();

    return problems;
}
