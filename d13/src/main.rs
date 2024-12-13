use rayon::prelude::*;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
struct State {
    x_remaining: i128,
    y_remaining: i128,
}

#[derive(Clone, Debug)]
struct Solution {
    a_presses: i128,
    b_presses: i128,
    total_cost: i128,
}

fn parse_button_data(input: &str) -> Vec<(i128, i128, i128, i128, i128, i128)> {
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

fn parse_button_line(line: &str, prefix: &str) -> Option<(i128, i128)> {
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
        .parse::<i128>()
        .ok()?;

    let y = coords
        .get(1)?
        .trim()
        .strip_prefix("Y+")?
        .parse::<i128>()
        .ok()?;

    Some((x, y))
}

fn parse_prize_line(line: &str) -> Option<(i128, i128)> {
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
        .parse::<i128>()
        .ok()?;

    let y = coords
        .get(1)?
        .trim()
        .strip_prefix("Y=")?
        .parse::<i128>()
        .ok()?;

    Some((x, y))
}

fn solve_claw(
    target_x: i128,
    target_y: i128,
    button_a_x: i128,
    button_a_y: i128,
    button_b_x: i128,
    button_b_y: i128,
    cost_a: i128,
    cost_b: i128,
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

    visited.insert(initial_state, 0);

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

            let new_cost = current_solution.total_cost + cost_a;

            if !visited.contains_key(&new_state) || visited.get(&new_state).unwrap() > &new_cost {
                visited.insert(new_state.clone(), new_cost);
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

            let new_cost = current_solution.total_cost + cost_b;

            if !visited.contains_key(&new_state) || visited.get(&new_state).unwrap() > &new_cost {
                visited.insert(new_state.clone(), new_cost);
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("input")?;
    let data = parse_button_data(&input);

    let tokens: i128 = data.par_iter()
        .map(|&(a_x, a_y, b_x, b_y, t_x, t_y)| {
            match solve_claw(
                t_x, // + 10000000000000,   // to execute the first part, comment out the number after +
                t_y, // + 10000000000000,   // to execute the first part, comment out the number after +
                a_x,
                a_y,
                b_x,
                b_y,
                3,
                1,
            ) {
                Some(solution) => solution.total_cost,
                None => 0,
            }
        })
        .sum();

    println!("{}", tokens);

    Ok(())
}