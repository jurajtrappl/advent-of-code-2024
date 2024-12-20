#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace0, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};
use std::collections::{HashSet, HashMap};
use rayon::prelude::*;
use std::sync::Mutex;

struct Onsen {
    patterns: HashSet<String>,
    desired_designs: Vec<String>
}

fn parse_pattern(input: &str) -> IResult<&str, String> {
    map(alpha1, String::from)(input)
}

fn parse_patterns(input: &str) -> IResult<&str, HashSet<String>> {
    map(
        separated_list1(
            delimited(multispace0, tag(","), multispace0),
            parse_pattern
        ),
        |patterns| patterns.into_iter().collect()
    )(input)
}

fn parse_designs(input: &str) -> IResult<&str, Vec<String>> {
    separated_list1(
        newline,
        map(alpha1, String::from)
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Onsen> {
    let (input, patterns) = parse_patterns(input)?;
    let (input, _) = tuple((newline, newline))(input)?;
    let (input, desired_designs) = parse_designs(input)?;
    Ok((input, Onsen { patterns, desired_designs }))
}

fn can_make_design(design: &str, patterns: &HashSet<String>, memo: &Mutex<HashMap<String, bool>>) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Ok(memo_guard) = memo.lock() {
        if let Some(&result) = memo_guard.get(design) {
            return result;
        }
    }

    for j in 1..=design.len() {
        let prefix = &design[..j];
        if patterns.contains(prefix) {
            if can_make_design(&design[j..], patterns, memo) {
                if let Ok(mut memo_guard) = memo.lock() {
                    _ = memo_guard.insert(design.to_string(), true);
                }
                return true;
            }
        }
    }

    if let Ok(mut memo_guard) = memo.lock() {
        _ = memo_guard.insert(design.to_string(), false);
    }
    false
}

fn count_ways(design: &str, patterns: &HashSet<String>, memo: &Mutex<HashMap<String, u64>>) -> u64 {
    if design.is_empty() {
        return 1;
    }

    if let Ok(memo_guard) = memo.lock() {
        if let Some(&count) = memo_guard.get(design) {
            return count;
        }
    }

    let mut total = 0;
    for i in 1..=design.len() {
        let prefix = &design[..i];
        if patterns.contains(prefix) {
            total += count_ways(&design[i..], patterns, memo);
        }
    }

    if let Ok(mut memo_guard) = memo.lock() {
        _ = memo_guard.insert(design.to_string(), total);
    }
    
    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("input")?;
    let (_, onsen) = parse_input(&input).map_err(|e| format!("Error: {}", e))?;
    
    let memo = Mutex::new(HashMap::new());
    
    let possible_designs: Vec<&String> = onsen.desired_designs
        .par_iter()
        .filter(|design| can_make_design(design, &onsen.patterns, &memo))
        .collect();
    let fst_part_result = possible_designs.len();

    let memo1 = Mutex::new(HashMap::new());
    let snd_part_result: u64 = possible_designs.par_iter().map(|design| count_ways(design, &onsen.patterns, &memo1)).sum();

    println!("First part: {}, Second part: {}", fst_part_result, snd_part_result);
    
    Ok(())
}