#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::HashSet;
use pathfinding::prelude::*;

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

    fn find_position(&self, tile_type: fn(&Tile) -> bool) -> (usize, usize) {
        self.cells
            .iter()
            .enumerate()
            .find_map(|(r, row)| {
                row.iter()
                    .enumerate()
                    .find(|(_, tile)| tile_type(tile))
                    .map(|(c, _)| (r, c))
            })
            .unwrap()
    }

    fn get_next_moves(&self, pos: (usize, usize), dir: (i32, i32)) -> Vec<((usize, usize), (i32, i32), usize)> {
        let next_dirs = match dir {
            (-1, 0) => vec![(-1, 0), (0, -1), (0, 1)],
            (0, 1) => vec![(0, 1), (-1, 0), (1, 0)],
            (1, 0) => vec![(1, 0), (0, -1), (0, 1)],
            (0, -1) => vec![(0, -1), (-1, 0), (1, 0)],
            _ => unreachable!(),
        };

        next_dirs
            .into_iter()
            .filter_map(|(dx, dy)| {
                let new_pos = ((pos.0 as i32 + dx) as usize, (pos.1 as i32 + dy) as usize);
                if !matches!(self.cells[new_pos.0][new_pos.1], Tile::Wall) {
                    let cost = if (dx, dy) == dir { 1 } else { 1000 };
                    Some((new_pos, (dx, dy), cost))
                } else {
                    None
                }
            })
            .collect()
    }

    fn solve(&self) -> (usize, usize) {
        let start = self.find_position(|t| matches!(t, Tile::Start));
        let end = self.find_position(|t| matches!(t, Tile::End));

        let paths = astar_bag(
            &(start, (0, 1)),
            |&(pos, dir)| {
                self.get_next_moves(pos, dir)
                    .into_iter()
                    .map(|(new_pos, new_dir, cost)| ((new_pos, new_dir), cost))
            },
            |_| 0,
            |&(pos, _)| pos == end,
        ).expect("should find a path");

        let optimal_tiles: HashSet<_> = paths.0
            .into_iter()
            .flat_map(|path| path.into_iter().map(|(pos, _)| pos))
            .collect();

        (paths.1, optimal_tiles.len())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("input")?;
    let maze = ReindeerMaze::new(&input);
    let (part1, part2) = maze.solve();
    
    println!("First part: {}, Second part: {}", part1, part2);
    Ok(())
}