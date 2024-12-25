#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use pathfinding::prelude::astar;
use rayon::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Pos(i32, i32);

impl Pos {
    fn neighbors(&self) -> Vec<Pos> {
        vec![
            Pos(self.0 + 1, self.1),
            Pos(self.0 - 1, self.1),
            Pos(self.0, self.1 + 1),
            Pos(self.0, self.1 - 1),
        ]
    }
}

#[derive(Debug)]
struct Grid {
    walls: HashSet<Pos>,
    start: Pos,
    end: Pos,
    width: i32,
    height: i32,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut walls = HashSet::new();
        let mut start = None;
        let mut end = None;

        let lines: Vec<&str> = input.lines().collect();
        let height = lines.len() as i32;
        let width = lines[0].len() as i32;

        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => {
                        _ = walls.insert(Pos(x as i32, y as i32));
                    }
                    'S' => {
                        start = Some(Pos(x as i32, y as i32));
                    }
                    'E' => {
                        end = Some(Pos(x as i32, y as i32));
                    }
                    _ => {}
                }
            }
        }

        Grid {
            walls,
            start: start.unwrap(),
            end: end.unwrap(),
            width,
            height,
        }
    }

    fn is_valid(&self, pos: &Pos) -> bool {
        pos.0 >= 0
            && pos.0 < self.width
            && pos.1 >= 0
            && pos.1 < self.height
            && !self.walls.contains(pos)
    }

    fn find_normal_path(&self) -> Option<(Vec<Pos>, i32)> {
        astar(
            &self.start,
            |p| {
                p.neighbors()
                    .into_iter()
                    .filter(|pos| self.is_valid(pos))
                    .map(|pos| (pos, 1))
            },
            |p| (p.0 - self.end.0).abs() + (p.1 - self.end.1).abs(),
            |p| *p == self.end,
        )
    }

    fn find_cheat_paths(&self, normal_path_cost: i32, max_cheat_distance: i32) -> Vec<i32> {
        let positions: Vec<(i32, i32)> = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .collect();

        positions
            .par_iter()
            .flat_map(|&(x1, y1)| {
                let start_pos = Pos(x1, y1);
                if self.walls.contains(&start_pos) {
                    return vec![];
                }

                positions
                    .iter()
                    .filter_map(|&(x2, y2)| {
                        let end_pos = Pos(x2, y2);
                        if self.walls.contains(&end_pos) {
                            return None;
                        }

                        let cheat_distance = (x2 - x1).abs() + (y2 - y1).abs();
                        if cheat_distance > max_cheat_distance {
                            return None;
                        }

                        let cost_to_cheat = astar(
                            &self.start,
                            |p| {
                                p.neighbors()
                                    .into_iter()
                                    .filter(|pos| self.is_valid(pos))
                                    .map(|pos| (pos, 1))
                            },
                            |p| (p.0 - start_pos.0).abs() + (p.1 - start_pos.1).abs(),
                            |p| *p == start_pos,
                        )?;

                        let cost_from_cheat = astar(
                            &end_pos,
                            |p| {
                                p.neighbors()
                                    .into_iter()
                                    .filter(|pos| self.is_valid(pos))
                                    .map(|pos| (pos, 1))
                            },
                            |p| (p.0 - self.end.0).abs() + (p.1 - self.end.1).abs(),
                            |p| *p == self.end,
                        )?;

                        let total_cost = cost_to_cheat.1 + cheat_distance + cost_from_cheat.1;
                        let saved = normal_path_cost - total_cost;
                        
                        if saved > 0 {
                            Some(saved)
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

fn solve(input: &str, min_savings: i32) -> (usize, usize) {
    let grid = Grid::parse(input);

    let (_, normal_cost) = grid.find_normal_path().unwrap();

    let fst_part_savings = grid.find_cheat_paths(normal_cost, 2);
    let snd_part_savings = grid.find_cheat_paths(normal_cost, 20);

    (
        fst_part_savings
            .into_iter()
            .filter(|&s| s >= min_savings)
            .count(),
        snd_part_savings
            .into_iter()
            .filter(|&s| s >= min_savings)
            .count(),
    )
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    let (fst_part_result, snd_part_result) = solve(&input, 100);
    println!(
        "First part: {}, Second part: {}",
        fst_part_result, snd_part_result
    );
}
