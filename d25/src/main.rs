#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

type Schematic = Vec<Vec<char>>;
type SchematicHeights = Vec<usize>;

fn schematic_col_heights(schematic: &Schematic) -> Vec<usize> {
    let transposed = aoe::transpose(schematic);
    let mut heights: Vec<usize> = Vec::new();
    for col in transposed.iter() {
        heights.push(col.iter().filter(|c| **c == '#').count() - 1);
    }
    heights
}

fn is_lock(schematic: &Schematic) -> bool {
    !schematic[0].contains(&'.') && !schematic.last().unwrap().contains(&'#')
}

fn key_fit_in_lock(key: &SchematicHeights, lock: &SchematicHeights) -> bool {
    key.iter()
        .zip(lock.iter())
        .all(|(&k, &l)| k + l <= 5)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("input")?;
    let parts: Vec<Vec<&str>> = input
        .split("\n\n")
        .map(|part| part.lines().collect())
        .collect();

    let mut keys: Vec<SchematicHeights> = Vec::new();
    let mut locks: Vec<SchematicHeights> = Vec::new();
    for part in parts {
        let schematic: Schematic = part.iter().map(|line| line.chars().collect()).collect();
        if is_lock(&schematic) {
            locks.push(schematic_col_heights(&schematic));
        } else {
            keys.push(schematic_col_heights(&schematic));
        }
    }

    let mut fst_part_result = 0;
    for key in keys.iter() {
        for lock in locks.iter() {
            fst_part_result += key_fit_in_lock(key, lock) as i32;
        }
    }

    println!("First part: {}", fst_part_result);

    Ok(())
}
