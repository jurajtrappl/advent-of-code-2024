#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::fmt;

const BOX: char = 'O';
const EMPTY: char = '.';
const WALL: char = '#';
const ROBOT: char = '@';

enum Tile {
    Box,
    Empty,
    Robot,
    Wall,
}

impl Tile {
    fn new(character: char) -> Tile {
        match character {
            BOX => Tile::Box,
            EMPTY => Tile::Empty,
            ROBOT => Tile::Robot,
            WALL => Tile::Wall,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::Box => write!(f, "{}", BOX),
            Tile::Empty => write!(f, "{}", EMPTY),
            Tile::Robot => write!(f, "{}", ROBOT),
            Tile::Wall => write!(f, "{}", WALL),
        }
    }
}

struct Direction {
    arrow: char,
    dx: i32,
    dy: i32,
}

impl Direction {
    fn new(arrow: char) -> Direction {
        match arrow {
            '^' => Direction { arrow, dx: -1, dy: 0 },
            '>' => Direction { arrow, dx: 0, dy: 1 },
            'v' => Direction { arrow, dx: 1, dy: 0 },
            '<' => Direction { arrow, dx: 0, dy: -1 },
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.arrow)
    }
}

struct Warehouse {
    maze: Vec<Vec<Tile>>,
    directions: Vec<Direction>,
}

impl Warehouse {
    fn new(input: &str) -> Warehouse {
        let parts: Vec<&str> = input.split("\n\n").collect();

        let maze: Vec<Vec<Tile>> = parts[0]
            .lines()
            .map(|l| l.chars().map(Tile::new).collect())
            .collect();
        let directions: Vec<Direction> = parts[1]
            .lines()
            .flat_map(|l| l.chars())
            .map(Direction::new)
            .collect();

        Warehouse { maze, directions }
    }

    fn box_gps(&self) -> usize {
        let mut result = 0;
        let rows = self.maze.len();
        let cols = self.maze[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if matches!(self.maze[r][c], Tile::Box) {
                    result += r * 100 + c;
                }
            }
        }

        result
    }

    fn find_boxes_before(&self, next_x: usize, next_y: usize, direction: &Direction) -> Vec<(usize, usize)> {
        // How many boxes are before the robot?
        let mut current_next_x = next_x;
        let mut current_next_y = next_y;
        let mut current_next = &self.maze[next_x][next_y];
        let mut boxes_before: Vec<(usize, usize)> = vec![(current_next_x, current_next_y)];
        while matches!(*current_next, Tile::Box) {
            let next_next_x = (current_next_x as i32 + direction.dx) as usize;
            let next_next_y = (current_next_y as i32 + direction.dy) as usize;
            let next_next = &self.maze[next_next_x][next_next_y];
            if !matches!(*next_next, Tile::Box) {
                break;
            }

            boxes_before.push((next_next_x, next_next_y));
            current_next_x = next_next_x;
            current_next_y = next_next_y;
            current_next = &self.maze[current_next_x][current_next_y];
        }

        boxes_before
    }

    fn find_robot(&self) -> Option<(usize, usize)> {
        let rows = self.maze.len();
        let cols = self.maze[0].len();

        for r in 0..rows {
            for c in 0..cols {
                if matches!(self.maze[r][c], Tile::Robot) {
                    return Some((r, c));
                }
            }
        }

        None
    }

    fn navigate(&mut self) {
        for direction in self.directions.iter() {
            let (current_x, current_y) = self.find_robot().unwrap();
            let next_x = (current_x as i32 + direction.dx) as usize;
            let next_y = (current_y as i32 + direction.dy) as usize;
            
            let next = &self.maze[next_x][next_y];
            if matches!(*next, Tile::Wall) {
                continue;
            }

            if matches!(*next, Tile::Empty) {
                self.maze[current_x][current_y] = Tile::Empty;
                self.maze[next_x][next_y] = Tile::Robot;
                continue;
            }

            let boxes_before: Vec<(usize, usize)> = self.find_boxes_before(next_x, next_y, direction);

            // Is there a free space after last for the robot to push?
            let &(last_box_before_x, last_box_before_y) = boxes_before.last().unwrap();
            let last_box_before_next_x = (last_box_before_x as i32 + direction.dx) as usize;
            let last_box_before_next_y = (last_box_before_y as i32 + direction.dy) as usize;
            let last_box_before_next = &self.maze[last_box_before_next_x][last_box_before_next_y];
            if matches!(*last_box_before_next, Tile::Empty) {
                // Push
                self.maze[last_box_before_next_x][last_box_before_next_y] = Tile::Box;
                self.maze[current_x][current_y] = Tile::Empty;
                self.maze[next_x][next_y] = Tile::Robot;
            }
        }
    }
}

impl fmt::Display for Warehouse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = self.maze.len();
        let cols = self.maze[0].len();

        for r in 0..rows {
            for c in 0..cols {
                write!(f, "{}", self.maze[r][c])?;
            }

            if r != rows - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("input")?;
    let mut warehouse = Warehouse::new(&input);
    println!("{}", warehouse);

    warehouse.navigate();
    let fst_part_result = warehouse.box_gps();

    println!("{}", fst_part_result);
    Ok(())
}