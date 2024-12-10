#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
use std::collections::{HashMap, HashSet};

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn to_vector(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    fn all() -> [Direction; 4] {
        [Direction::Up, Direction::Right, Direction::Down, Direction::Left]
    }
}

type Position = (i32, i32);
type Path = Vec<Position>;
type HeightMap = Vec<Vec<u32>>;

struct TrailMap {
    height_map: HeightMap,
    rows: i32,
    cols: i32,
}

impl TrailMap {
    fn new(input: &str) -> Self {
        let height_map: HeightMap = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let rows = height_map.len() as i32;
        let cols = height_map[0].len() as i32;
        
        Self { height_map, rows, cols }
    }

    fn get_height(&self, pos: Position) -> u32 {
        self.height_map[pos.0 as usize][pos.1 as usize]
    }

    fn find_trailheads(&self) -> Vec<Position> {
        let mut trailheads = Vec::new();
        for r in 0..self.rows {
            for c in 0..self.cols {
                if self.get_height((r, c)) == 0 {
                    trailheads.push((r, c));
                }
            }
        }
        trailheads
    }

    fn get_valid_moves(&self, pos: Position) -> Vec<Position> {
        let current_height = self.get_height(pos);
        
        Direction::all()
            .iter()
            .map(|dir| {
                let dir_vec = dir.to_vector();
                (pos.0 + dir_vec.0, pos.1 + dir_vec.1)
            })
            .filter(|&new_pos| !aoe::is_out_of_bounds(new_pos, self.rows, self.cols))
            .filter(|&new_pos| self.get_height(new_pos) == current_height + 1)
            .collect()
    }

    fn find_all_paths(&self) -> Vec<Path> {
        let mut queue: Vec<Path> = self.find_trailheads()
            .into_iter()
            .map(|pos| vec![pos])
            .collect();
        let mut finished = Vec::new();

        while let Some(path) = queue.pop() {
            let last_pos = *path.last().unwrap();
            
            if self.get_height(last_pos) == 9 {
                finished.push(path);
                continue;
            }

            for next_pos in self.get_valid_moves(last_pos) {
                let mut new_path = path.clone();
                new_path.push(next_pos);
                queue.push(new_path);
            }
        }
        
        finished
    }
}

fn calculate_trailhead_scores(paths: &[Path]) -> usize {
    let mut trailhead_scores: HashMap<Position, HashSet<Position>> = HashMap::new();
    
    for path in paths {
        _ = trailhead_scores
            .entry(path[0])
            .or_insert_with(HashSet::new)
            .insert(*path.last().unwrap());
    }
    
    trailhead_scores.values().map(|set| set.len()).sum()
}

fn calculate_trailhead_ratings(paths: &[Path]) -> usize {
    let mut ratings: HashMap<Position, usize> = HashMap::new();
    
    for path in paths {
        *ratings.entry(path[0]).or_insert(0) += 1;
    }
    
    ratings.values().sum()
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    let trail_map = TrailMap::new(&input);
    let all_paths = trail_map.find_all_paths();
    
    let score = calculate_trailhead_scores(&all_paths);
    let rating = calculate_trailhead_ratings(&all_paths);
    
    println!("First part: {}, Second part: {}", score, rating);
}