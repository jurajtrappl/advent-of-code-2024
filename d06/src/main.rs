#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashSet;
use std::time::Instant;
use rayon::prelude::*;

const OBSTRUCTION: char = '#';
const START: char = '^';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn forward_vector(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
}

fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            if grid[r][c] == START {
                return Some((r, c));
            }
        }
    }
    None
}

fn count_visited_positions(grid: &Vec<Vec<char>>) -> i32 {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let start = match find_start(&grid) {
        Some(start) => start,
        None => unreachable!(),
    };

    let mut current_position = (start.0 as i32, start.1 as i32);
    let mut current_direction = Direction::Up;
    let mut visited = HashSet::new();
    _ = visited.insert((current_position.0, current_position.1));

    loop {
        let (dx, dy) = current_direction.forward_vector();
        let next_position = (current_position.0 + dx, current_position.1 + dy);

        if aoe::is_out_of_bounds(next_position, rows, cols) {
            break;
        }

        if grid[next_position.0 as usize][next_position.1 as usize] == OBSTRUCTION {
            current_direction = current_direction.turn_right();
        } else {
            current_position = next_position;
            _ = visited.insert((current_position.0, current_position.1));
        }
    }

    visited.len() as i32
}

fn check_for_true_cycles(grid: &Vec<Vec<char>>) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    let start = match find_start(&grid) {
        Some(start) => start,
        None => unreachable!(),
    };

    let mut current_position = (start.0 as i32, start.1 as i32);
    let mut current_direction = Direction::Up;
    let mut visited = HashSet::new();
    _ = visited.insert((current_position.0, current_position.1, current_direction));

    loop {
        let (dx, dy) = current_direction.forward_vector();
        let next_position = (current_position.0 + dx, current_position.1 + dy);

        if visited.contains(&(next_position.0, next_position.1, current_direction)) {
            return true;
        }

        if aoe::is_out_of_bounds(next_position, rows, cols) {
            break;
        }

        if grid[next_position.0 as usize][next_position.1 as usize] == OBSTRUCTION {
            current_direction = current_direction.turn_right();
        } else {
            current_position = next_position;
            _ = visited.insert((current_position.0, current_position.1, current_direction));
        }
    }

    false
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    let grid: Vec<Vec<char>> = input.split("\n").map(|l| l.chars().collect()).collect();
    
    let start = Instant::now();
    let first_result = count_visited_positions(&grid);
    let duration = start.elapsed();
    println!("First part: {} (took: {:?})", first_result, duration);

    let start = Instant::now();
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    
    let positions: Vec<_> = (0..rows)
        .flat_map(|r| (0..cols).map(move |c| (r, c)))
        .filter(|&(r, c)| {
            grid[r as usize][c as usize] != OBSTRUCTION 
            && grid[r as usize][c as usize] != START
        })
        .collect();

    let snd_part_counter = positions.par_iter()
        .map(|&(r, c)| {
            let mut grid = grid.clone();
            grid[r as usize][c as usize] = OBSTRUCTION;
            check_for_true_cycles(&grid) as i32
        })
        .sum::<i32>();

    let duration = start.elapsed();
    println!("Second part: {} (took: {:?})", snd_part_counter, duration);
}
