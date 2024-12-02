#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

fn split_location_lists(locations: &Vec<Vec<i32>>) -> (Vec<i32>, Vec<i32>) {
    let (mut fst, mut snd): (Vec<i32>, Vec<i32>) = locations
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

fn find_distance(fst_locations: &[i32], snd_locations: &[i32]) -> i32 {
    fst_locations
        .iter()
        .zip(snd_locations.iter())
        .map(|(fst, snd)| (fst - snd).abs())
        .sum()
}

fn calculate_similarity_score(fst_locations: &[i32], snd_locations: &[i32]) -> i32 {
    let counter = aoe::count_occurrences(snd_locations);
    fst_locations.iter().fold(0, |acc, elem| {
        acc + elem * counter.get(elem).map_or(0, |&count| count as i32)
    })
}

fn solve(input: &str, op: fn(&[i32], &[i32]) -> i32) -> i32 {
    let locations: Vec<Vec<i32>> = aoe::parse_to_2dvec(input);
    let (fst_locations, snd_locations): (Vec<i32>, Vec<i32>) = split_location_lists(&locations);
    op(&fst_locations, &snd_locations)
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    println!(
        "First part: {}. Second part: {}",
        solve(&input, find_distance),
        solve(&input, calculate_similarity_score)
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_solve_example_fst_part() {
        let input = aoe::read_input_file("example_input").unwrap();
        assert_eq!(11, super::solve(&input, super::find_distance));
    }

    #[test]
    fn test_solve_fst_part() {
        let input = aoe::read_input_file("input").unwrap();
        assert_eq!(2000468, super::solve(&input, super::find_distance));
    }

    #[test]
    fn test_solve_example_snd_part() {
        let input = aoe::read_input_file("example_input").unwrap();
        assert_eq!(31, super::solve(&input, super::calculate_similarity_score));
    }

    #[test]
    fn test_solve_snd_part() {
        let input = aoe::read_input_file("input").unwrap();
        assert_eq!(18567089, super::solve(&input, super::calculate_similarity_score));
    }
}
