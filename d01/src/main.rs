#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use aoe::{count_occurrences, parse_to_2dvec, read_input_file};

fn split_location_lists(locations: &Vec<Vec<u32>>) -> (Vec<u32>, Vec<u32>) {
    let (mut fst, mut snd): (Vec<u32>, Vec<u32>) = locations
        .iter()
        .map(|l| match l.as_slice() {
            [a, b, ..] => (*a, *b),
            _ => panic!("Each locations list row should have 2 numbers."),
        })
        .unzip();

    fst.sort();
    snd.sort();

    (fst, snd)
}

fn find_distance(fst_locations: &[u32], snd_locations: &[u32]) -> u32 {
    fst_locations
        .iter()
        .zip(snd_locations.iter())
        .map(|(fst, snd)| if fst > snd { fst - snd } else { snd - fst })
        .sum()
}

fn calculate_similarity_score(fst_locations: &[u32], snd_locations: &[u32]) -> u32 {
    let counter = count_occurrences(snd_locations);
    fst_locations.iter().fold(0, |acc, elem| {
        acc + elem * counter.get(elem).map_or(0, |&count| count as u32)
    })
}

fn solve(input: &str, op: fn(&[u32], &[u32]) -> u32) -> u32 {
    let locations: Vec<Vec<u32>> = parse_to_2dvec(input);
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
    use crate::{calculate_similarity_score, find_distance, read_input_file, solve};

    #[test]
    fn test_solve_example() {
        let input = read_input_file("example_input").unwrap();
        assert_eq!(11, solve(&input, find_distance));
    }

    #[test]
    fn test_solve_fst_part() {
        let input = read_input_file("input").unwrap();
        assert_eq!(2000468, solve(&input, find_distance));
    }

    #[test]
    fn test_solve_snd_part() {
        let input = read_input_file("input").unwrap();
        assert_eq!(18567089, solve(&input, calculate_similarity_score));
    }
}
