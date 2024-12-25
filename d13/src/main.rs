#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::{HashMap, VecDeque};

const EPSILON: f64 = 1e-10;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    x_remaining: i64,
    y_remaining: i64,
}

#[derive(Clone, Debug)]
struct Solution {
    a_presses: i64,
    b_presses: i64,
    total_cost: i64,
}

fn parse_button_data(input: &str) -> Vec<(i64, i64, i64, i64, i64, i64)> {
    let mut result = Vec::new();
    let mut lines = input.lines();

    while let Some(button_a) = lines.next() {
        if let Some((x1, y1)) = parse_button_line(button_a, "Button A:") {
            if let Some(button_b) = lines.next() {
                if let Some((x2, y2)) = parse_button_line(button_b, "Button B:") {
                    if let Some(prize) = lines.next() {
                        if let Some((px, py)) = parse_prize_line(prize) {
                            result.push((x1, y1, x2, y2, px, py));
                        }
                    }
                }
            }

            lines.next();
        }
    }
    result
}

fn parse_button_line(line: &str, prefix: &str) -> Option<(i64, i64)> {
    if !line.starts_with(prefix) {
        return None;
    }

    let coords = line
        .strip_prefix(prefix)?
        .trim()
        .split(',')
        .collect::<Vec<&str>>();

    let x = coords
        .get(0)?
        .trim()
        .strip_prefix("X+")?
        .parse::<i64>()
        .ok()?;

    let y = coords
        .get(1)?
        .trim()
        .strip_prefix("Y+")?
        .parse::<i64>()
        .ok()?;

    Some((x, y))
}

fn parse_prize_line(line: &str) -> Option<(i64, i64)> {
    if !line.starts_with("Prize:") {
        return None;
    }

    let coords = line
        .strip_prefix("Prize:")?
        .trim()
        .split(',')
        .collect::<Vec<&str>>();

    let x = coords
        .get(0)?
        .trim()
        .strip_prefix("X=")?
        .parse::<i64>()
        .ok()?;

    let y = coords
        .get(1)?
        .trim()
        .strip_prefix("Y=")?
        .parse::<i64>()
        .ok()?;

    Some((x, y))
}

fn solve_claw_fst(
    target_x: i64,
    target_y: i64,
    button_a_x: i64,
    button_a_y: i64,
    button_b_x: i64,
    button_b_y: i64,
) -> Option<Solution> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();

    let initial_state = State {
        x_remaining: target_x,
        y_remaining: target_y,
    };

    queue.push_back((
        initial_state.clone(),
        Solution {
            a_presses: 0,
            b_presses: 0,
            total_cost: 0,
        },
    ));

    _ = visited.insert(initial_state, 0);

    while let Some((state, current_solution)) = queue.pop_front() {
        if state.x_remaining == 0 && state.y_remaining == 0 {
            return Some(current_solution);
        }

        if (button_a_x > 0 && state.x_remaining > 0)
            || (button_a_x < 0 && state.x_remaining < 0)
            || (button_a_y > 0 && state.y_remaining > 0)
            || (button_a_y < 0 && state.y_remaining < 0)
        {
            let new_state = State {
                x_remaining: state.x_remaining - button_a_x,
                y_remaining: state.y_remaining - button_a_y,
            };

            let new_cost = current_solution.total_cost + 3;

            if !visited.contains_key(&new_state) || visited.get(&new_state).unwrap() > &new_cost {
                _ = visited.insert(new_state.clone(), new_cost);
                queue.push_back((
                    new_state,
                    Solution {
                        a_presses: current_solution.a_presses + 1,
                        b_presses: current_solution.b_presses,
                        total_cost: new_cost,
                    },
                ));
            }
        }

        if (button_b_x > 0 && state.x_remaining > 0)
            || (button_b_x < 0 && state.x_remaining < 0)
            || (button_b_y > 0 && state.y_remaining > 0)
            || (button_b_y < 0 && state.y_remaining < 0)
        {
            let new_state = State {
                x_remaining: state.x_remaining - button_b_x,
                y_remaining: state.y_remaining - button_b_y,
            };

            let new_cost = current_solution.total_cost + 1;

            if !visited.contains_key(&new_state) || visited.get(&new_state).unwrap() > &new_cost {
                _ = visited.insert(new_state.clone(), new_cost);
                queue.push_back((
                    new_state,
                    Solution {
                        a_presses: current_solution.a_presses,
                        b_presses: current_solution.b_presses + 1,
                        total_cost: new_cost,
                    },
                ));
            }
        }
    }

    None
}

fn solve_claw_snd(t_x: f64, t_y: f64, a_x: f64, a_y: f64, b_x: f64, b_y: f64) -> Option<i64> {
    // [ a_x b_x ]
    // [ a_y b_y ]

    let det: f64 = a_x * b_y - b_x * a_y;
    if det == 0.0 {
        return None;
    }

    // cramer's rule
    let x: f64 = (t_x * b_y - b_x * t_y) / det;
    let y: f64 = (a_x * t_y - t_x * a_y) / det;
    if x < 0.0 || y < 0.0 || (x - x.round()).abs() >= EPSILON || (y - y.round()).abs() >= EPSILON {
        return None;
    }

    Some((3.0 * x + 1.0 * y) as i64)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("input")?;
    let data = parse_button_data(&input);

    let tokens_fst: i64 = data.iter()
        .map(|&(a_x, a_y, b_x, b_y, t_x, t_y)| {
            match solve_claw_fst(
                t_x,
                t_y,
                a_x,
                a_y,
                b_x,
                b_y,
            ) {
                Some(solution) => solution.total_cost,
                None => 0,
            }
        })
        .sum();

    let tokens_snd: i64 = data
        .iter()
        .map(|&(a_x, a_y, b_x, b_y, t_x, t_y)| {
            match solve_claw_snd(
                (t_x + 10000000000000) as f64,
                (t_y + 10000000000000) as f64,
                a_x as f64,
                a_y as f64,
                b_x as f64,
                b_y as f64,
            ) {
                Some(total_cost) => total_cost,
                None => 0,
            }
        })
        .sum();

    println!("First part: {}, Second part: {}", tokens_fst, tokens_snd);

    Ok(())
}
