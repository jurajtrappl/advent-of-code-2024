#![warn(
    bad_style,
    unused,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

use std::collections::{HashMap, HashSet};
use petgraph::{Graph, Undirected};

fn bron_kerbosch(
    graph: &Graph<&str, (), Undirected>,
    r: &mut Vec<petgraph::prelude::NodeIndex>,
    p: &mut Vec<petgraph::prelude::NodeIndex>,
    x: &mut Vec<petgraph::prelude::NodeIndex>,
    max_clique: &mut Vec<petgraph::prelude::NodeIndex>
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            max_clique.clear();
            max_clique.extend(r.iter());
        }
        return;
    }

    let p_copy = p.clone();    
    for &v in &p_copy {
        let neighbors: HashSet<_> = graph.neighbors(v).collect();
        
        let mut new_r = r.clone();
        new_r.push(v);
        
        let mut new_p: Vec<_> = p.iter()
            .filter(|&&n| neighbors.contains(&n))
            .cloned()
            .collect();
            
        let mut new_x: Vec<_> = x.iter()
            .filter(|&&n| neighbors.contains(&n))
            .cloned()
            .collect();

        bron_kerbosch(graph, &mut new_r, &mut new_p, &mut new_x, max_clique);
        
        p.retain(|&n| n != v);
        x.push(v);
    }
}

fn find_triangles(graph: &Graph<&str, (), Undirected>) -> usize {
    let mut triangles = HashSet::new();
    for node_a in graph.node_indices() {
        for node_b in graph.neighbors(node_a) {
            for node_c in graph.neighbors(node_b) {
                if graph.contains_edge(node_a, node_c) {
                    let mut nodes = [
                        graph[node_a],
                        graph[node_b],
                        graph[node_c],
                    ];

                    if nodes.iter().any(|&node| node.starts_with('t')) {
                        nodes.sort();
                        _ = triangles.insert(nodes);
                    }
                }
            }
        }
    }

    triangles.len()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut nodes = HashMap::new();
    let mut graph = Graph::<&str, (), Undirected>::new_undirected();
    
    let input = aoe::read_input_file("input")?;
    for line in input.lines() {
        let [a, b] = line.split("-").collect::<Vec<_>>()[..] else { continue; };
        let a_idx = *nodes.entry(a).or_insert_with(|| graph.add_node(a));
        let b_idx = *nodes.entry(b).or_insert_with(|| graph.add_node(b));
        _ = graph.add_edge(a_idx, b_idx, ());
    }

    let fst_part_result = find_triangles(&graph);
    
    let mut r = Vec::new();
    let mut p: Vec<_> = graph.node_indices().collect();
    let mut x = Vec::new();
    let mut max_clique = Vec::new();

    bron_kerbosch(&graph, &mut r, &mut p, &mut x, &mut max_clique);

    let mut result: Vec<_> = max_clique.iter().map(|&idx| graph[idx]).collect();
    result.sort();
    
    let password = result.join(",");
    
    println!("First part: {}, Second part: {}", fst_part_result, password);

    Ok(())
}