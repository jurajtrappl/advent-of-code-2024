#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use regex::Regex;

fn solve(input: &str) -> (i64, i64) {
    let mul_re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();
    
    let mut operations: Vec<(usize, String)> = Vec::new();
    
    for capture in mul_re.captures_iter(input) {
        let pos = capture.get(0).unwrap().start();
        let num1: i64 = capture[1].parse().unwrap();
        let num2: i64 = capture[2].parse().unwrap();
        operations.push((pos, format!("mul,{},{}", num1, num2)));
    }
    
    for m in do_re.find_iter(input) {
        operations.push((m.start(), "do".to_string()));
    }
    
    for m in dont_re.find_iter(input) {
        operations.push((m.start(), "dont".to_string()));
    }

    operations.sort_by_key(|k| k.0);
    
    let mut enabled = true;
    let (mut fst_part_sum, mut snd_part_sum) = (0, 0);
    
    for (_, op) in operations {
        let parts: Vec<&str> = op.split(',').collect();
        match parts[0] {
            "do" => enabled = true,
            "dont" => enabled = false,
            "mul" => {
                let num1: i64 = parts[1].parse().unwrap();
                let num2: i64 = parts[2].parse().unwrap();
                fst_part_sum += num1 * num2;
                if enabled {
                    snd_part_sum += num1 * num2;
                }
            },
            _ => {}
        }
    }

    return (fst_part_sum, snd_part_sum)
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    let (fst_part_sum, snd_part_sum) = solve(&input); 
    println!("First part: {}, Second part: {}", fst_part_sum, snd_part_sum);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_example() {
        let example_input = aoe::read_input_file("example_input").unwrap();
        let (fst_part_sum, _) = super::solve(&example_input);
        let example_input1 = aoe::read_input_file("example_input1").unwrap();
        let (_, snd_part_sum) = super::solve(&example_input1);

        assert_eq!(161, fst_part_sum);
        assert_eq!(48, snd_part_sum);
    }

    #[test]
    fn test_solve() {
        let input = aoe::read_input_file("input").unwrap();
        let (fst_part_sum, snd_part_sum) = super::solve(&input);
        
        assert_eq!(175615763, fst_part_sum);
        assert_eq!(74361272, snd_part_sum);
    }
}