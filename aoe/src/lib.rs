use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::Read;

pub fn read_input_file(filename: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut buffer = String::new();
    _ = file.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn count_occurrences<T: Hash + Eq>(items: &[T]) -> HashMap<&T, usize> {
    let mut counts = HashMap::new();
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }
    counts
}

pub fn parse_to_2dvec<T>(input: &str) -> Vec<Vec<T>>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Debug,
{
    input
        .lines()
        .map(|l| {
            l.trim()
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect()
}

pub fn transpose<T>(original: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    let rows = original.len();
    let cols = original[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| original[row][col].clone()).collect())
        .collect()
}

pub fn is_out_of_bounds(pos: (i32, i32), rows: i32, cols: i32) -> bool {
    pos.0 < 0 || pos.0 >= rows || pos.1 < 0 || pos.1 >= cols
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_input_uint() {
        assert_eq!(vec![vec![1u8, 2u8], vec![3u8, 4u8]], super::parse_to_2dvec::<u8>("1 2\n3 4"));
    }

    #[test]
    fn test_parse_str() {
        assert_eq!(vec![vec!["1", "2"], vec!["3", "4"]], super::parse_to_2dvec::<String>("1 2\n3 4"));
    }

    #[test]
    fn test_transpose() {
        assert_eq!(
            vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]],
            super::transpose(&vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
        )
    }
}