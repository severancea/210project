// ds210 final project
// andrew severance | asev@bu.edu

// analysis.rs
use petgraph::algo::dijkstra;
use petgraph::graph::{NodeIndex, Graph};

#[derive(Debug, serde::Deserialize)]
pub struct RaceData {
    pub race_id: usize,
    pub driver_id: usize,
    pub driver_ref: String,
}

pub fn avg_dist(graph: &mut Graph<String, ()>) -> f64 {
    let start_node = graph.add_node("Start".to_string());

    let mut total_distance = 0.0;
    let mut total_pairs = 0;

    for node in graph.node_indices() {
        if node != start_node {
            let distances = dijkstra_distance(graph, start_node, node);
            if distances[0] != 0 && distances[0] != usize::MAX {
                total_distance += distances.iter().map(|&d| d as f64).sum::<f64>();
                total_pairs += distances.len();
            }
        }
    }

    if total_pairs > 0 {
        total_distance / total_pairs as f64
    } else {
        0.0
    }
}

pub fn dijkstra_distance(graph: &Graph<String, ()>, start: NodeIndex, end: NodeIndex) -> Vec<usize> {
    let distances = dijkstra(graph, start, Some(end), |_| 1);

    if let Some(distance) = distances.get(&end) {
        vec![*distance]
    } else {
        vec![usize::MAX]
    }
}