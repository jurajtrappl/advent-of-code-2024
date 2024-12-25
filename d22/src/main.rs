#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashSet;
use rayon::prelude::*;

fn next_secret(current: u128) -> u128 {
    let mult = current * 64;
    let mix = mult ^ current;
    let prune = mix % 16777216;
    
    let div = prune / 32;
    let mix1 = div ^ prune;
    let prune1 = mix1 % 16777216;
    
    let mult1 = prune1 * 2048;
    let mix2 = mult1 ^ prune1;
    let prune2 = mix2 % 16777216;
    
    prune2
}

fn generate_secrets(initial: u128, count: usize) -> Vec<u128> {
    let mut secrets = Vec::with_capacity(count + 1);
    let mut current = initial;
    
    secrets.push(current);
    for _ in 0..count {
        current = next_secret(current);
        secrets.push(current);
    }
    
    secrets
}

fn secrets_to_prices(secrets: &[u128]) -> Vec<i32> {
    secrets.iter()
        .map(|&n| (n % 10) as i32)
        .collect()
}

fn calculate_changes(prices: &[i32]) -> Vec<i32> {
    prices.windows(2)
        .map(|w| w[1] - w[0])
        .collect()
}

fn find_pattern_price(changes: &[i32], pattern: &[i32], prices: &[i32]) -> Option<i32> {
    changes.windows(4).enumerate()
        .find(|(_, window)| *window == pattern)
        .map(|(i, _)| prices[i + 4])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("example_input")?;
    let initial_numbers: Vec<u128> = input
        .lines()
        .map(|line| line.parse::<u128>().unwrap())
        .collect();

    let fst_part_result: u128 = initial_numbers.par_iter()
        .map(|&initial| generate_secrets(initial, 2000)[2000])
        .sum();
    println!("First part: {}", fst_part_result);

    let mut all_patterns = HashSet::new();
    let all_buyer_data: Vec<_> = initial_numbers.par_iter()
        .map(|&initial| {
            let secrets = generate_secrets(initial, 2000);
            let prices = secrets_to_prices(&secrets);
            let changes = calculate_changes(&prices);
            
            let mut patterns = HashSet::new();
            for window in changes.windows(4) {
                _ = patterns.insert(window.to_vec());
            }
            
            ((prices, changes), patterns)
        })
        .collect();

    for (_, patterns) in &all_buyer_data {
        all_patterns.extend(patterns.iter().cloned());
    }

    let buyer_data: Vec<_> = all_buyer_data.into_iter()
        .map(|(data, _)| data)
        .collect();

    let (best_pattern, max_bananas) = all_patterns.par_iter()
        .map(|pattern| {
            let total_bananas = buyer_data.iter()
                .filter_map(|(prices, changes)| find_pattern_price(changes, pattern, prices))
                .sum::<i32>();
            (pattern.clone(), total_bananas)
        })
        .max_by_key(|&(_, bananas)| bananas)
        .unwrap();

    println!("Second part - the best pattern: {:?}, maximum bananas: {}", best_pattern, max_bananas);
    Ok(())
}