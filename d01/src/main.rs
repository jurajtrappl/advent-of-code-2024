#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use aoe::{count_occurrences, read_input_file};

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    let content: String = input.chars().collect();
    content
        .split("\n")
        .map(|l| {
            l.trim()
                .split(" ")
                .filter(|p| !p.is_empty())
                .map(|n| n.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn split_location_lists(locations: &Vec<Vec<u32>>) -> (Vec<u32>, Vec<u32>) {
    let (mut fst, mut snd): (Vec<u32>, Vec<u32>) = locations.iter().map(|l| (l[0], l[1])).unzip();

    fst.sort();
    snd.sort();

    (fst, snd)
}

fn find_distance(fst_locations: &Vec<u32>, snd_locations: &Vec<u32>) -> u32 {
    fst_locations
        .iter()
        .zip(snd_locations.iter())
        .map(|(fst, snd)| if fst > snd { fst - snd } else { snd - fst })
        .sum()
}

fn calculate_similarity_score(fst_locations: &Vec<u32>, snd_locations: &Vec<u32>) -> u32 {
    let counter = count_occurrences(snd_locations);
    fst_locations.iter().fold(0, |acc, e| {
        acc + e * (*counter.get(&e).unwrap_or(&0) as u32)
    })
}

fn solve(input: &str, op: fn(&Vec<u32>, &Vec<u32>) -> u32) -> u32 {
    let locations: Vec<Vec<u32>> = parse_input(input);
    let (fst_locations, snd_locations): (Vec<u32>, Vec<u32>) = split_location_lists(&locations);
    op(&fst_locations, &snd_locations)
}

fn main() {
    let input = read_input_file("input").unwrap();
    println!(
        "First part: {}. Second part: {}",
        solve(&input, find_distance),
        solve(&input, calculate_similarity_score)
    )
}

#[cfg(test)]
mod tests {
    use crate::{find_distance, read_input_file, solve};

    #[test]
    fn test_example_solve_fst_part() {
        let input = read_input_file("example_input").unwrap();
        assert_eq!(11, solve(&input, find_distance));
    }
}
