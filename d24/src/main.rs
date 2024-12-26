#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
struct GateConnection {
    first_gate: String,
    second_gate: String,
    operation: fn(bool, bool) -> bool,
    output_gate: String,
}

fn parse_input(input: &str) -> (HashMap<String, bool>, Vec<GateConnection>) {
    let parts: Vec<&str> = input.split("\n\n").collect();

    let gates: HashMap<String, bool> = parts[0]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let value = match parts[1] {
                "0" => false,
                _ => true,
            };
            (parts[0].to_string(), value)
        })
        .collect();
    let gates_connections: Vec<GateConnection> = parts[1]
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            let operation: fn(bool, bool) -> bool = match parts[1] {
                "AND" => |a, b| a & b,
                "OR" => |a, b| a | b,
                "XOR" => |a, b| a ^ b,
                _ => unreachable!(),
            };
            GateConnection {
                first_gate: parts[0].to_string(),
                second_gate: parts[2].to_string(),
                operation,
                output_gate: parts[4].to_string(),
            }
        })
        .collect();

    (gates, gates_connections)
}

fn calculate_expected_sum(x_bits: &[bool], y_bits: &[bool]) -> Vec<bool> {
    let mut result = Vec::new();
    let mut carry = false;
    let max_len = x_bits.len().max(y_bits.len());

    for i in 0..max_len {
        let x = x_bits.get(i).copied().unwrap_or(false);
        let y = y_bits.get(i).copied().unwrap_or(false);

        let sum = x ^ y ^ carry;
        carry = (x & y) | (x & carry) | (y & carry);

        result.push(sum);
    }
    if carry {
        result.push(true);
    }
    result
}

fn extract_bits(gates: &HashMap<String, bool>, prefix: char) -> Vec<bool> {
    let mut bits: Vec<_> = gates
        .iter()
        .filter(|(k, _)| k.starts_with(prefix))
        .collect();
    bits.sort_by(|(a, _), (b, _)| {
        let a_num = a[1..].parse::<usize>().unwrap();
        let b_num = b[1..].parse::<usize>().unwrap();
        a_num.cmp(&b_num)
    });
    bits.into_iter().map(|(_, &v)| v).collect()
}

fn evaluate_circuit(
    gates: &HashMap<String, bool>,
    connections: &[GateConnection],
) -> HashMap<String, bool> {
    let mut wire_values = gates.clone();
    let mut evaluated = HashSet::new();

    loop {
        let mut made_progress = false;
        
        let new_evaluations: Vec<_> = connections.par_iter()
            .enumerate()
            .filter(|(_, conn)| !evaluated.contains(&conn.output_gate))
            .filter_map(|(i, conn)| {
                if let (Some(&input1), Some(&input2)) = (
                    wire_values.get(&conn.first_gate),
                    wire_values.get(&conn.second_gate),
                ) {
                    Some((i, (conn.operation)(input1, input2)))
                } else {
                    None
                }
            })
            .collect();

        for (i, result) in new_evaluations {
            let conn = &connections[i];
            _ = wire_values.insert(conn.output_gate.clone(), result);
            _ = evaluated.insert(conn.output_gate.clone());
            made_progress = true;
        }

        if !made_progress {
            break;
        }
    }
    wire_values
}

fn is_correct_addition(gates: &HashMap<String, bool>, connections: &[GateConnection]) -> bool {
    let wire_values = evaluate_circuit(gates, connections);

    let x_bits = extract_bits(gates, 'x');
    let y_bits = extract_bits(gates, 'y');
    let expected_sum = calculate_expected_sum(&x_bits, &y_bits);

    let mut z_bits: Vec<_> = wire_values
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect();
    z_bits.sort_by(|(a, _), (b, _)| {
        let a_num = a[1..].parse::<usize>().unwrap();
        let b_num = b[1..].parse::<usize>().unwrap();
        a_num.cmp(&b_num)
    });

    let actual_sum: Vec<_> = z_bits.into_iter().map(|(_, &v)| v).collect();
    actual_sum == expected_sum
}

fn try_swaps(gates: &HashMap<String, bool>, connections: &[GateConnection]) -> Option<Vec<String>> {
    let output_wires: HashSet<_> = connections
        .iter()
        .map(|conn| conn.output_gate.clone())
        .collect();

    let found = std::sync::Arc::new(std::sync::Mutex::new(None));
    let found_clone = found.clone();

    _ = output_wires.iter()
        .combinations(8)
        .par_bridge()
        .any(|swap_candidates| {
            if found.lock().unwrap().is_some() {
                return true;
            }

            let success = swap_candidates.chunks(2)
                .permutations(4)
                .par_bridge()
                .any(|pairs| {
                    let mut modified_connections = connections.to_vec();

                    for pair in pairs {
                        let wire1 = &pair[0];
                        let wire2 = &pair[1];

                        for conn in &mut modified_connections {
                            if &conn.output_gate == *wire1 {
                                conn.output_gate = (*wire2).clone();
                            } else if &conn.output_gate == *wire2 {
                                conn.output_gate = (*wire1).clone();
                            }
                        }
                    }

                    if is_correct_addition(gates, &modified_connections) {
                        let mut swapped_wires: Vec<String> = swap_candidates
                            .iter()
                            .map(|s| (*s).clone())
                            .collect();
                        swapped_wires.sort();
                        *found_clone.lock().unwrap() = Some(swapped_wires);
                        true
                    } else {
                        false
                    }
                });

            success
        });

    found.lock().unwrap().take()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("input")?;
    let (gates, gates_connections) = parse_input(&input);

    let wire_values = evaluate_circuit(&gates, &gates_connections);
    let mut z_wires: Vec<_> = wire_values
        .iter()
        .filter(|(name, _)| name.starts_with('z'))
        .collect();
    z_wires.sort_by(|(a, _), (b, _)| {
        let a_num = a[1..].parse::<usize>().unwrap();
        let b_num = b[1..].parse::<usize>().unwrap();
        a_num.cmp(&b_num)
    });
    let mut result = 0u64;
    for (_, value) in z_wires.iter().rev() {
        result = (result << 1) | ((**value) as u64);
    }
    println!("First part: {}", result);

    if let Some(swapped_wires) = try_swaps(&gates, &gates_connections) {
        println!("Second part: {}", swapped_wires.join(","));
    } else {
        println!("No solution found");
    }

    Ok(())
}
