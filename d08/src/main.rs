#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::{HashMap, HashSet};

trait VectorMovement {
    fn sub(&self, other: &Self) -> (i32, i32);
    fn add(&self, other: &Self) -> (i32, i32);
}

impl VectorMovement for (i32, i32) {
    fn sub(&self, other: &Self) -> (i32, i32) {
        (self.0 - other.0, self.1 - other.1)
    }

    fn add(&self, other: &Self) -> (i32, i32) {
        (self.0 + other.0, self.1 + other.1)
    }
}

fn get_antenna_pairs(locations: &Vec<(i32, i32)>) -> Vec<((i32, i32), (i32, i32))> {
    let mut pairs = Vec::new();
    for i in 0..locations.len() {
        for j in i + 1..locations.len() {
            pairs.push((locations[i], locations[j]));
        }
    }
    pairs
}

fn is_out_of_bounds(pos: (i32, i32), rows: i32, cols: i32) -> bool {
    pos.0 < 0 || pos.0 >= rows || pos.1 < 0 || pos.1 >= cols
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    let city_map: Vec<Vec<char>> = input.split("\n").map(|l| l.chars().collect()).collect();
    let rows = city_map.len() as i32;
    let cols = city_map[0].len() as i32;

    let mut antenna_locations: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for r in 0..rows {
        for c in 0..cols {
            if city_map[r as usize][c as usize] != '.' {
                antenna_locations
                    .entry(city_map[r as usize][c as usize])
                    .or_insert(Vec::new())
                    .push((r as i32, c as i32));
            }
        }
    }

    let mut unique_locations = HashSet::new();
    let mut unique_locations_2 = HashSet::new();

    for frequency in antenna_locations.keys() {
        let locations = get_antenna_pairs(&antenna_locations.get(frequency).unwrap());
        for (fst, snd) in locations.iter() {
            let fst_to_snd = snd.sub(fst);
            let snd_to_fst = fst.sub(snd);

            // the first part
            let new_fst = fst.add(&fst_to_snd).add(&fst_to_snd);
            let new_snd = snd.add(&snd_to_fst).add(&snd_to_fst);

            if !is_out_of_bounds(new_fst, rows, cols) {
                _ = unique_locations.insert(new_fst);
                
            }

            if !is_out_of_bounds(new_snd, rows, cols) {
                _ = unique_locations.insert(new_snd);
            }

            // the second part
            let mut curr_fst = *fst;
            let mut curr_snd = *snd;

            _ = unique_locations_2.insert(*fst);
            _ = unique_locations_2.insert(*snd);

            for _ in 0..100 {
                curr_fst = curr_fst.add(&fst_to_snd);
                if !is_out_of_bounds(curr_fst, rows, cols) {
                    _ = unique_locations_2.insert(curr_fst);
                }
            }

            for _ in 0..100 {
                curr_snd = curr_snd.add(&snd_to_fst);
                if !is_out_of_bounds(curr_snd, rows, cols) {
                    _ = unique_locations_2.insert(curr_snd);
                }
            }
        }
    }

    println!("First part: {}, Second part: {}", unique_locations.len(), unique_locations_2.len());
}
