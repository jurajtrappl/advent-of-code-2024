#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::{HashMap, HashSet};

#[derive(Debug)]
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

fn evaluate_circuit(gates: &HashMap<String, bool>, connections: &[GateConnection]) -> u64 {
    let mut wire_values = gates.clone();
    let mut evaluated = HashSet::new();

    loop {
        let mut made_progress = false;

        for conn in connections {
            if evaluated.contains(&conn.output_gate) {
                continue;
            }

            if let (Some(&input1), Some(&input2)) = (
                wire_values.get(&conn.first_gate),
                wire_values.get(&conn.second_gate),
            ) {
                let result = (conn.operation)(input1, input2);
                _ = wire_values.insert(conn.output_gate.clone(), result);
                _ = evaluated.insert(conn.output_gate.clone());
                made_progress = true;
            }
        }

        if !made_progress {
            break;
        }
    }

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

    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = aoe::read_input_file("input")?;
    let (gates, gates_connections) = parse_input(&input);

    let fst_part_result = evaluate_circuit(&gates, &gates_connections);
    println!("First part: {}", fst_part_result);

    Ok(())
}
