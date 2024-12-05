#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::{HashMap, HashSet, VecDeque};

fn topological_sort(graph: &HashMap<u32, HashSet<u32>>, nodes: &[u32]) -> Option<Vec<u32>> {
    let mut in_degree: HashMap<u32, usize> = nodes.iter().map(|&n| (n, 0)).collect();
    
    for edges in graph.values() {
        for &dest in edges {
            *in_degree.get_mut(&dest).unwrap() += 1;
        }
    }
    
    let mut queue: VecDeque<u32> = in_degree
        .iter()
        .filter(|&(_, &deg)| deg == 0)
        .map(|(&n, _)| n)
        .collect();
        
    let mut sorted = Vec::new();
    
    while let Some(node) = queue.pop_front() {
        sorted.push(node);
        
        if let Some(edges) = graph.get(&node) {
            for &dest in edges {
                let deg = in_degree.get_mut(&dest).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(dest);
                }
            }
        }
    }
    
    (sorted.len() == nodes.len()).then_some(sorted)
}

fn build_graph(nodes: &[u32], pairs: &[(u32, u32)]) -> HashMap<u32, HashSet<u32>> {
    let mut graph: HashMap<u32, HashSet<u32>> = nodes.iter().map(|&n| (n, HashSet::new())).collect();
    
    for &(x, y) in pairs {
        if nodes.contains(&x) && nodes.contains(&y) {
            _ = graph.get_mut(&x).unwrap().insert(y);
        }
    }
    
    graph
}

fn main() {
    let input = aoe::read_input_file("input").unwrap();
    let sections: Vec<&str> = input.split("\n\n").collect();

    let pairs: Vec<(u32, u32)> = sections[0]
        .split('\n')
        .map(|p| {
            let nums: Vec<u32> = p.split('|').map(|n| n.parse().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect();
        
    let rules: Vec<Vec<u32>> = sections[1]
        .split('\n')
        .map(|r| r.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    let valid_rules: Vec<&Vec<u32>> = rules
        .iter()
        .filter(|&rule| {
            let graph = build_graph(rule, &pairs);
            matches!(topological_sort(&graph, rule), Some(sorted) if &sorted == rule)
        })
        .collect();

    let corrected: Vec<Vec<u32>> = rules
        .iter()
        .filter(|r| !valid_rules.contains(r))
        .filter_map(|rule| {
            let graph = build_graph(rule, &pairs);
            topological_sort(&graph, rule)
        })
        .collect();
        
    println!("First part: {}", valid_rules.iter().map(|r| r[r.len() / 2]).sum::<u32>());
    println!("Second part: {}", corrected.iter().map(|r| r[r.len() / 2]).sum::<u32>());
}