#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashMap;

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    let sections: Vec<&str> = input.split("\n\n").collect();

    let number_pairs: Vec<(u32, u32)> = sections[0]
        .split("\n")
        .map(|p| {
            let nums: Vec<u32> = p.split("|").map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();
    let update_rules: Vec<Vec<u32>> = sections[1]
        .split("\n")
        .map(|r| r.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut must_come_before: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut must_come_after: HashMap<u32, Vec<u32>> = HashMap::new();
    
    for (x, y) in number_pairs.iter() {
        must_come_before.entry(*x).or_insert(Vec::new()).push(*y);
        must_come_after.entry(*y).or_insert(Vec::new()).push(*x);
    }

    let mut correct_rules = Vec::new();
    for rule in update_rules.iter() {
        let mut is_correct = true;
        
        for i in 0..rule.len() {
            let current = &rule[i];
            let numbers_before = &rule[..i];
            let numbers_after = &rule[i + 1..];

            if let Some(required_after) = must_come_before.get(current) {
                for later_num in numbers_after {
                    if !required_after.contains(later_num) {
                        is_correct = false;
                        break;
                    }
                }
            }
            
            if let Some(required_before) = must_come_after.get(current) {
                for earlier_num in numbers_before {
                    if !required_before.contains(earlier_num) {
                        is_correct = false;
                        break;
                    }
                }
            }
        }

        if is_correct {
            correct_rules.push(rule);
        }
    }

    let middle_page_number: u32 = correct_rules.iter().map(|r| r[r.len() / 2]).sum();
    println!("{}", middle_page_number);
}
