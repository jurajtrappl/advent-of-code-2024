#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

type Position = (usize, usize);

use std::collections::{HashMap, HashSet, VecDeque};

fn find_letter_regions(grid: &Vec<Vec<char>>) -> Vec<HashSet<Position>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = HashSet::new();
    let mut regions = Vec::new();

    for r in 0..height {
        for c in 0..width {
            if visited.contains(&(r, c)) {
                continue;
            }

            let mut region = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((r, c));
            let target_char = grid[r][c];

            while let Some((curr_r, curr_c)) = queue.pop_front() {
                if !visited.insert((curr_r, curr_c)) {
                    continue;
                }

                _ = region.insert((curr_r, curr_c));

                let neighbors = [
                    (curr_r, curr_c.wrapping_sub(1)),
                    (curr_r, curr_c + 1),
                    (curr_r.wrapping_sub(1), curr_c),
                    (curr_r + 1, curr_c),
                ];

                for &(next_r, next_c) in &neighbors {
                    if next_r < height
                        && next_c < width
                        && !visited.contains(&(next_r, next_c))
                        && grid[next_r][next_c] == target_char
                    {
                        queue.push_back((next_r, next_c));
                    }
                }
            }

            regions.push(region);
        }
    }

    regions
}

fn count_neighbors(positions: &HashSet<Position>) -> HashMap<Position, usize> {
    let pos_set: HashSet<_> = positions.iter().copied().collect();
    positions
        .iter()
        .copied()
        .map(|(row, col)| {
            let count = [
                (row, col.wrapping_sub(1)),
                (row, col + 1),
                (row.wrapping_sub(1), col),
                (row + 1, col),
            ]
            .iter()
            .filter(|&&pos| pos_set.contains(&pos))
            .count();
            ((row, col), count)
        })
        .collect()
}

fn count_corners(region: &HashSet<Position>) -> usize {
    let mut sides = 0;

    for &(x, y) in region.iter() {
        // outer corners
        if !region.contains(&(x.wrapping_sub(1), y)) && !region.contains(&(x, y.wrapping_sub(1))) {
            sides += 1;
        }
        if !region.contains(&(x.wrapping_sub(1), y)) && !region.contains(&(x, y + 1)) {
            sides += 1;
        }
        if !region.contains(&(x + 1, y)) && !region.contains(&(x, y.wrapping_sub(1))) {
            sides += 1;
        }
        if !region.contains(&(x + 1, y)) && !region.contains(&(x, y + 1)) {
            sides += 1;
        }

        // inner corners
        if region.contains(&(x.wrapping_sub(1), y))
            && region.contains(&(x, y.wrapping_sub(1)))
            && !region.contains(&(x.wrapping_sub(1), y.wrapping_sub(1)))
        {
            sides += 1;
        }
        if region.contains(&(x + 1, y))
            && region.contains(&(x, y.wrapping_sub(1)))
            && !region.contains(&(x + 1, y.wrapping_sub(1)))
        {
            sides += 1;
        }
        if region.contains(&(x + 1, y))
            && region.contains(&(x, y + 1))
            && !region.contains(&(x + 1, y + 1))
        {
            sides += 1;
        }
        if region.contains(&(x.wrapping_sub(1), y))
            && region.contains(&(x, y + 1))
            && !region.contains(&(x.wrapping_sub(1), y + 1))
        {
            sides += 1;
        }
    }

    sides
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("input")?;
    let garden: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let regions = find_letter_regions(&garden);

    let mut total_price_fst_part = 0;
    let mut total_price_snd_part = 0;

    for region in regions.iter() {
        let area = region.len();
        let neighbors = count_neighbors(&region);

        let mut perimeter_fst_part = 0;
        for (_, count) in neighbors.iter() {
            perimeter_fst_part += 4 - *count;
        }
        total_price_fst_part += area * perimeter_fst_part;

        total_price_snd_part += area * count_corners(&region);
    }

    println!(
        "First part: {}, Second part: {}",
        total_price_fst_part, total_price_snd_part
    );

    Ok(())
}
