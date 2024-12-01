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
