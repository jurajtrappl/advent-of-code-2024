#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
use rayon::prelude::*;

enum Op {
    Add,
    Mul,
    Concat,
}

fn apply_op(x: u128, y: u128, op: &Op) -> u128 {
    match op {
        Op::Add => x + y,
        Op::Mul => x * y,
        Op::Concat => (x.to_string() + &y.to_string()).parse().unwrap(),
    }
}

fn parse_equations(input: &str) -> Vec<(u128, Vec<u128>)> {
    input
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(": ").collect();
            let equation_result = parts[0].parse::<u128>().unwrap();
            let operands: Vec<u128> = parts[1]
                .split_whitespace()
                .map(|n| n.parse::<u128>().unwrap())
                .collect();
            (equation_result, operands)
        })
        .collect()
}

fn solve(calibration_equations: &Vec<(u128, Vec<u128>)>, operators: Vec<Op>) -> u128 {
    calibration_equations
        .par_iter()
        .filter_map(|(calibration_result, operands)| {
            let mut intermediate_results: Vec<u128> = vec![operands[0]];

            for &operand in operands.iter().skip(1) {
                let mut new_results = Vec::new();
                for &intermediate_result in intermediate_results.iter() {
                    for op in operators.iter() {
                        let op_result = apply_op(intermediate_result, operand, &op);
                        if op_result <= *calibration_result {
                            new_results.push(op_result);
                        }
                    }
                }
                intermediate_results = new_results;
            }

            if intermediate_results
                .iter()
                .any(|&v| v == *calibration_result)
            {
                Some(*calibration_result)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    let calibration_equations: Vec<(u128, Vec<u128>)> = parse_equations(&input);

    let fst_part_result = solve(&calibration_equations, vec![Op::Add, Op::Mul]);
    let snd_part_result = solve(&calibration_equations, vec![Op::Add, Op::Mul, Op::Concat]);
    println!("First part: {}, Second part: {}", fst_part_result, snd_part_result);
}

#[cfg(test)]
mod tests {
    use super::{Op, parse_equations, solve};

    #[test]
    fn test_solve() {
        let input = aoe::read_input_file("input").unwrap();
        let example_input = aoe::read_input_file("example_input").unwrap();
        let input_calibration_equations: Vec<(u128, Vec<u128>)> = parse_equations(&input);
        let example_input_calibration_equations: Vec<(u128, Vec<u128>)> = parse_equations(&example_input);

        assert_eq!(3749, solve(&example_input_calibration_equations, vec![Op::Add, Op::Mul]));
        assert_eq!(2299996598890, solve(&input_calibration_equations, vec![Op::Add, Op::Mul]));

        assert_eq!(11387, solve(&example_input_calibration_equations, vec![Op::Add, Op::Mul, Op::Concat]));
        assert_eq!(362646859298554, solve(&input_calibration_equations, vec![Op::Add, Op::Mul, Op::Concat]));
    }
}