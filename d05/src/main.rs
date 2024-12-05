#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::{HashMap, HashSet, VecDeque};

fn topological_sort(graph: &HashMap<u32, HashSet<u32>>, nodes: &[u32]) -> Option<Vec<u32>> {
    let mut in_degree: HashMap<u32, usize> = HashMap::new();
    let nodes: HashSet<u32> = nodes.iter().cloned().collect();

    for &node in &nodes {
        _ = in_degree.insert(node, 0);
    }

    for &node in &nodes {
        if let Some(edges) = graph.get(&node) {
            for &dest in edges {
                if nodes.contains(&dest) {
                    *in_degree.get_mut(&dest).unwrap() += 1;
                }
            }
        }
    }

    let mut queue: VecDeque<u32> = in_degree
        .iter()
        .filter(|&(_, &degree)| degree == 0)
        .map(|(&node, _)| node)
        .collect();

    let mut result = Vec::new();

    while let Some(node) = queue.pop_front() {
        result.push(node);

        if let Some(edges) = graph.get(&node) {
            for &dest in edges {
                if nodes.contains(&dest) {
                    let degree = in_degree.get_mut(&dest).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(dest);
                    }
                }
            }
        }
    }

    if result.len() == nodes.len() {
        Some(result)
    } else {
        None // cycle
    }
}

fn build_local_graph(rule: &[u32], number_pairs: &[(u32, u32)]) -> HashMap<u32, HashSet<u32>> {
    let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();

    for &num in rule {
        _ = graph.insert(num, HashSet::new());
    }

    for &(x, y) in number_pairs {
        if rule.contains(&x) && rule.contains(&y) {
            _ = graph.get_mut(&x).unwrap().insert(y);
        }
    }

    graph
}

fn check_rule(rule: &[u32], number_pairs: &[(u32, u32)]) -> Option<Vec<u32>> {
    let local_graph = build_local_graph(rule, number_pairs);
    topological_sort(&local_graph, rule)
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    let sections: Vec<&str> = input.split("\n\n").collect();

    let number_pairs: Vec<(u32, u32)> = sections[0]
        .split("\n")
        .map(|p| {
            let nums: Vec<u32> = p.split("|").map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();
    let update_rules: Vec<Vec<u32>> = sections[1]
        .split("\n")
        .map(|r| r.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    let mut must_come_before: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut must_come_after: HashMap<u32, Vec<u32>> = HashMap::new();

    for (x, y) in number_pairs.iter() {
        must_come_before.entry(*x).or_insert(Vec::new()).push(*y);
        must_come_after.entry(*y).or_insert(Vec::new()).push(*x);
    }

    // the first part
    let mut correct_rules = Vec::new();
    for rule in update_rules.iter() {
        let mut is_correct = true;

        for i in 0..rule.len() {
            let current = &rule[i];
            let numbers_before = &rule[..i];
            let numbers_after = &rule[i + 1..];

            if let Some(required_after) = must_come_before.get(current) {
                for later_num in numbers_after {
                    if !required_after.contains(later_num) {
                        is_correct = false;
                        break;
                    }
                }
            }

            if let Some(required_before) = must_come_after.get(current) {
                for earlier_num in numbers_before {
                    if !required_before.contains(earlier_num) {
                        is_correct = false;
                        break;
                    }
                }
            }

            if !is_correct {
                break;
            }
        }

        if is_correct {
            correct_rules.push(rule);
        }
    }

    let middle_page_number: u32 = correct_rules.iter().map(|r| r[r.len() / 2]).sum();
    println!("First part: {}", middle_page_number);

    // the second part
    let incorrect_rules: Vec<&Vec<u32>> = update_rules
        .iter()
        .filter(|r| !correct_rules.contains(r))
        .collect();
    let mut corrected_rules: Vec<Vec<u32>> = Vec::new();
    for rule in incorrect_rules {
        let result = check_rule(rule, &number_pairs);
        if let Some(sorted) = result {
            corrected_rules.push(sorted);
        }
    }

    let middle_page_number2: u32 = corrected_rules.iter().map(|r| r[r.len() / 2]).sum();
    println!("Second part: {}", middle_page_number2);
}
