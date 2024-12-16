#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

const EMPTY: char = '.';
const END: char = 'E';
const START: char = 'S';
const WALL: char = '#';

#[derive(Debug)]
enum Tile {
    Empty,
    End,
    Start,
    Wall,
}

impl Tile {
    fn new(character: char) -> Tile {
        match character {
            EMPTY => Tile::Empty,
            END => Tile::End,
            START => Tile::Start,
            WALL => Tile::Wall,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    position: (usize, usize),
    direction: (i32, i32),
    score: usize,
    path: Vec<(usize, usize)>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // reverse ordering to make a min-heap
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct ReindeerMaze {
    cells: Vec<Vec<Tile>>,
}

impl ReindeerMaze {
    fn new(input: &str) -> ReindeerMaze {
        let cells = input
            .lines()
            .map(|l| l.chars().map(Tile::new).collect())
            .collect();

        ReindeerMaze { cells }
    }

    fn find_start(&self) -> (usize, usize) {
        self.cells
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, ref tile)| matches!(tile, Tile::Start))
                    .map(|(c, _)| (r, c))
            })
            .unwrap()
    }

    fn find_end(&self) -> (usize, usize) {
        self.cells
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, ref tile)| matches!(tile, Tile::End))
                    .map(|(c, _)| (r, c))
            })
            .unwrap()
    }

    fn find_least_scored_path_score(&self, start: (usize, usize), end: (usize, usize)) -> usize {
        let height = self.cells.len();
        let width = self.cells[0].len();
        let mut min_scores = vec![vec![vec![usize::MAX; 4]; width]; height];

        let mut queue = BinaryHeap::new();
        queue.push(State {
            position: start,
            direction: (0, 1),
            score: 0,
            path: vec![start],
        });

        while let Some(state) = queue.pop() {
            if state.position == end {
                return state.score;
            }

            let dir_idx = match state.direction {
                (0, 1) => 0,
                (1, 0) => 1,
                (0, -1) => 2,
                (-1, 0) => 3,
                _ => unreachable!(),
            };

            if state.score >= min_scores[state.position.0][state.position.1][dir_idx] {
                continue;
            }
            min_scores[state.position.0][state.position.1][dir_idx] = state.score;

            for (new_pos, new_dir) in self.neighbors(&state) {
                let rotation_cost = if new_dir != state.direction { 1000 } else { 0 };
                let new_score = state.score + 1 + rotation_cost;

                let mut new_path = state.path.clone();
                new_path.push(new_pos);

                queue.push(State {
                    position: new_pos,
                    direction: new_dir,
                    score: new_score,
                    path: new_path,
                });
            }
        }

        unreachable!("No path found")
    }

    fn find_optimal_tiles(
        &self,
        target_score: usize,
        start: (usize, usize),
        end: (usize, usize),
    ) -> HashSet<(usize, usize)> {
        let mut optimal_tiles = HashSet::new();
        _ = optimal_tiles.insert(start);
        _ = optimal_tiles.insert(end);

        let mut queue = BinaryHeap::new();
        queue.push(State {
            position: start,
            direction: (0, 1),
            score: 0,
            path: vec![start],
        });

        while let Some(state) = queue.pop() {
            if state.score > target_score {
                // cant be optimal, optimal paths reach with exactly target score (from p1)
                continue;
            }

            if state.position == end {
                if state.score == target_score {
                    // add all tiles in this optimal path
                    optimal_tiles.extend(state.path.iter());
                }
                continue;
            }

            for (new_pos, new_dir) in self.neighbors(&state) {
                let rotation_cost = if new_dir != state.direction { 1000 } else { 0 };
                let new_score = state.score + 1 + rotation_cost;

                if new_score <= target_score {
                    let mut new_path = state.path.clone();
                    new_path.push(new_pos);

                    queue.push(State {
                        position: new_pos,
                        direction: new_dir,
                        score: new_score,
                        path: new_path,
                    });
                }
            }
        }

        optimal_tiles
    }

    fn neighbors(&self, state: &State) -> Vec<((usize, usize), (i32, i32))> {
        let all_neighbors = match state.direction {
            (-1, 0) => vec![(-1, 0), (0, -1), (0, 1)],
            (0, 1) => vec![(0, 1), (-1, 0), (1, 0)],
            (1, 0) => vec![(1, 0), (0, -1), (0, 1)],
            (0, -1) => vec![(0, -1), (-1, 0), (1, 0)],
            _ => unreachable!(),
        };

        all_neighbors
            .into_iter()
            .filter_map(|(dx, dy)| {
                let new_position_x = (state.position.0 as i32 + dx) as usize;
                let new_position_y = (state.position.1 as i32 + dy) as usize;

                if !matches!(self.cells[new_position_x][new_position_y], Tile::Wall) {
                    Some(((new_position_x, new_position_y), (dx, dy)))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("example_input")?;
    let maze = ReindeerMaze::new(&input);

    let start = maze.find_start();
    let end = maze.find_end();

    let fst_part_result = maze.find_least_scored_path_score(start, end);
    let optimal_tiles = maze.find_optimal_tiles(fst_part_result, start, end);
    let snd_part_result = optimal_tiles.len();
    println!(
        "First part: {}, Second part: {}",
        fst_part_result, snd_part_result
    );

    Ok(())
}
