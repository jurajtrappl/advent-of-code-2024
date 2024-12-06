#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up
        }
    }

    fn forward_vector(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1)
        }
    }
}

fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == '^' {
                return Some((r, c))
            }
        }
    }

    None
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    let mut grid: Vec<Vec<char>> = input.split("\n").map(|l| l.chars().collect()).collect();
    let rows = grid.len() as i32;
    let cols = grid.len() as i32;

    let start = match find_start(&grid) {
        Some(start) => start,
        None => unreachable!()
    };
    println!("Starting position: {}, {}", start.0, start.1);

    let mut current_position = (start.0 as i32, start.1 as i32);
    let mut current_direction = Direction::Up;
    let mut visited = HashSet::new();
    visited.insert((current_position.0, current_position.1));

    loop {
        let (dx, dy) = current_direction.forward_vector();
        let next_position = (current_position.0 + dx, current_position.1 + dy);

        if next_position.0 < 0 || next_position.0 >= rows || next_position.1 < 0 || next_position.1 >= cols {
            break;
        }

        if grid[next_position.0 as usize][next_position.1 as usize] == '#' {
            current_direction = current_direction.turn_right();
        } else {
            current_position = next_position;
            visited.insert((current_position.0, current_position.1));
        }
    }

    println!("{}", visited.len());

}
