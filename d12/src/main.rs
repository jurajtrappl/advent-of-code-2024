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
                    if next_r < height
                        && next_c < width
                        && !visited.contains(&(next_r, next_c))
                        && grid[next_r][next_c] == target_char
                    {
                        queue.push_back((next_r, next_c));
                    }
                }
            }

            groups.push(group);
        }
    }

    groups
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

fn get_corner_points(cell: &Position) -> Vec<(i32, i32)> {
    let (row, col) = *cell;
    vec![
        (row as i32, col as i32),           // Top-left
        (row as i32, col as i32 + 1),       // Top-right
        (row as i32 + 1, col as i32 + 1),   // Bottom-right
        (row as i32 + 1, col as i32),       // Bottom-left
    ]
}

fn count_sides(group: &HashSet<Position>) -> usize {
    // Convert cells to corner points
    let mut corner_points = HashSet::new();
    for cell in group {
        for corner in get_corner_points(cell) {
            corner_points.insert(corner);
        }
    }

    // Remove interior points
    let cells_set: HashSet<_> = group.iter().collect();
    corner_points.retain(|&(y, x)| {
        // Skip points where any potential cell would be out of bounds
        if y == 0 || x == 0 {
            return true;
        }

        let potential_cells = [
            ((y - 1) as usize, (x - 1) as usize),
            ((y - 1) as usize, x as usize),
            (y as usize, (x - 1) as usize),
            (y as usize, x as usize),
        ];
        
        let shared_cells = potential_cells.iter()
            .filter(|&&pos| cells_set.contains(&pos))
            .count();
            
        shared_cells < 4
    });

    // Convert to Vec and sort for traversal
    let mut points: Vec<_> = corner_points.into_iter().collect();
    if points.len() < 2 {
        return points.len();
    }

    // Find starting point (leftmost, then topmost)
    let start = *points.iter()
        .min_by_key(|&&(y, x)| (x, y))
        .unwrap();
    
    // Build path by always choosing the next counterclockwise point
    let mut path = vec![start];
    let mut current = start;
    let mut visited = HashSet::new();
    visited.insert(start);

    while visited.len() < points.len() {
        // Get all unvisited adjacent points
        let mut candidates: Vec<_> = points.iter()
            .filter(|&&p| !visited.contains(&p))
            .filter(|&&(y, x)| {
                let dy = y as i32 - current.0 as i32;
                let dx = x as i32 - current.1 as i32;
                dy.abs() <= 1 && dx.abs() <= 1 && (dy != 0 || dx != 0)
            })
            .collect();

        if candidates.is_empty() {
            break;
        }

        // Sort candidates by angle (counterclockwise)
        let prev = if path.len() >= 2 {
            path[path.len() - 2]
        } else {
            (current.0, current.1 - 1) // Assume we came from the left
        };

        candidates.sort_by_key(|&&(y, x)| {
            let prev_dy = current.0 as i32 - prev.0 as i32;
            let prev_dx = current.1 as i32 - prev.1 as i32;
            let next_dy = y as i32 - current.0 as i32;
            let next_dx = x as i32 - current.1 as i32;
            
            // Calculate angle between vectors using cross product
            let cross = prev_dx * next_dy - prev_dy * next_dx;
            let dot = prev_dx * next_dx + prev_dy * next_dy;
            
            // Return sorting key (prefer left turns)
            (-cross, -dot)
        });

        // Take the first candidate (most counterclockwise)
        let next = *candidates[0];
        visited.insert(next);
        path.push(next);
        current = next;
    }

    // Add first point to close the path
    path.push(start);

    // Count direction changes
    let mut sides = 0;
    for i in 0..path.len() - 1 {
        let curr = path[i];
        let next = path[i + 1];
        
        let dx = next.1 as i32 - curr.1 as i32;
        let dy = next.0 as i32 - curr.0 as i32;
        
        if i == 0 {
            sides += 1;
        } else {
            let prev = path[i - 1];
            let prev_dx = curr.1 as i32 - prev.1 as i32;
            let prev_dy = curr.0 as i32 - prev.0 as i32;
            
            if prev_dx != dx || prev_dy != dy {
                sides += 1;
            }
        }
    }

    sides
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("example_input")?;
    let garden: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let groups = find_letter_groups(&garden);

    let mut total_price_fst_part = 0;
    let mut total_price_snd_part = 0;

    for group in groups.iter() {
        let area = group.len();
        let neighbors = count_neighbors(&group);

        // First part
        let mut perimeter_fst_part = 0;
        for (_, count) in neighbors.iter() {
            perimeter_fst_part += 4 - *count;
        }
        total_price_fst_part += area * perimeter_fst_part;

        // Second part
        let sides = count_sides(&group);
        println!("Area * Sides = {} * {}", area, sides);
        total_price_snd_part += area * sides;
    }

    println!(
        "First part: {}, Second part: {}",
        total_price_fst_part, total_price_snd_part
    );

    Ok(())
}
