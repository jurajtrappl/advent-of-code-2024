#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashMap;
use std::error::Error;

struct Stones {
    count: usize,
    number_counts: HashMap<u64, usize>,
}

impl Stones {
    fn new(input: &str) -> Self {
        let number_counts = input
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .fold(HashMap::new(), |mut acc, stone| {
                *acc.entry(stone).or_insert(0) += 1;
                acc
            });

        Self {
            count: number_counts.values().sum(),
            number_counts,
        }
    }

    fn blink(&mut self) {
        let mut new_counts = HashMap::new();

        for (&stone, &count) in &self.number_counts {
            match stone {
                0 => *new_counts.entry(1).or_insert(0) += count,
                n => {
                    let digits = n.to_string();
                    if digits.len() % 2 == 0 {
                        let mid = digits.len() / 2;
                        let left = digits[..mid].parse::<u64>().unwrap();
                        let right = digits[mid..].parse::<u64>().unwrap();
                        *new_counts.entry(left).or_insert(0) += count;
                        *new_counts.entry(right).or_insert(0) += count;
                    } else {
                        *new_counts.entry(n * 2024).or_insert(0) += count;
                    }
                }
            }
        }

        self.number_counts = new_counts;
        self.count = self.number_counts.values().sum();
    }

    fn simulate_blinks(&mut self, times: u32) {
        for _ in 0..times {
            self.blink();
        }
    }

    fn get_count(&self) -> usize {
        self.count
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = aoe::read_input_file("input")?;
    let mut stones = Stones::new(&input);

    stones.simulate_blinks(25);
    println!("First part: {}", stones.get_count());

    stones.simulate_blinks(50);
    println!("Second part: {}", stones.get_count());

    Ok(())
}