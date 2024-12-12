#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

type Position = (usize, usize);

use std::collections::{HashMap, HashSet, VecDeque};

fn find_letter_groups(grid: &Vec<Vec<char>>) -> Vec<HashSet<Position>> {
    let height = grid.len();
    let width = grid[0].len();
    let mut visited = HashSet::new();
    let mut groups = Vec::new();

    for r in 0..height {
        for c in 0..width {
            if visited.contains(&(r, c)) {
                continue;
            }

            let mut group = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((r, c));
            let target_char = grid[r][c];

            while let Some((curr_r, curr_c)) = queue.pop_front() {
                if !visited.insert((curr_r, curr_c)) {
                    continue;
                }
                
                _ = group.insert((curr_r, curr_c));

                let neighbors = [
                    (curr_r, curr_c.wrapping_sub(1)),
                    (curr_r, curr_c + 1),
                    (curr_r.wrapping_sub(1), curr_c),
                    (curr_r + 1, curr_c),
                ];

                for &(next_r, next_c) in &neighbors {
                    if next_r < height && next_c < width && 
                       !visited.contains(&(next_r, next_c)) && 
                       grid[next_r][next_c] == target_char {
                        queue.push_back((next_r, next_c));
                    }
                }
            }

            groups.push(group);
        }
    }

    groups
}

fn count_perimeter(positions: &HashSet<Position>) -> HashMap<Position, usize> {
    let pos_set: HashSet<_> = positions.iter().copied().collect();
    positions.iter().copied()
        .map(|(row, col)| {
            let count = [(row, col.wrapping_sub(1)), (row, col + 1), 
                        (row.wrapping_sub(1), col), (row + 1, col)]
                .iter()
                .filter(|&&pos| pos_set.contains(&pos))
                .count();
            ((row, col), 4 - count)
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("input")?;
    
    let garden: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let groups = find_letter_groups(&garden);

    let mut total_price = 0;
    for group in groups.iter() {
        let area = group.len();
        let perimeter: usize = count_perimeter(&group).values().sum();
        let price = area * perimeter;
        total_price += price;
    }

    println!("First part: {}", total_price);

    Ok(())
}
