// ds210 final project
// andrew severance | asev@bu.edu
// collaborators on ideas and coding: none
// outside resources: chatgpt used for debugging assistance and test implementation

// main.rs
mod data;
mod models;
mod graph;
mod analysis;

use std::fs::File;
use csv::ReaderBuilder;
use std::error::Error;
use std::collections::HashSet;
use std::collections::HashMap;
use petgraph::adj::NodeIndex;
use petgraph::graph::DiGraph;
use petgraph::dot::Dot;
use petgraph::dot::Config;
use petgraph::adj::IndexType;
use crate::analysis::avg_dist;
use crate::analysis::dijkstra_distance;

#[derive(Debug, serde::Deserialize)]
struct RaceData {
    race_id: usize,
    driver_id: usize,
    driver_ref: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addpilot() {
        let mut f1_graph = F1Graph::new();
        let pilot_name = "Pilot";
        let pilot_node = f1_graph.add_pilot(pilot_name);

        assert_eq!(f1_graph.graph.node_weight(pilot_node), Some(&pilot_name.to_string()));
    }

    #[test]
    fn test_avgdist() {
        let mut f1_graph = F1Graph::new();

        let a = f1_graph.add_pilot("Pilot A");
        let b = f1_graph.add_pilot("Pilot B");
        let c = f1_graph.add_pilot("Pilot C");

        f1_graph.add_race_edge(a, b);
        f1_graph.add_race_edge(b, c);

        let average_distance = avg_dist(&mut f1_graph.graph);

        assert_eq!(average_distance, 1.0);
    }

    #[test]
    fn test_dijkstra() {
        let mut f1_graph = F1Graph::new();

        let a = f1_graph.add_pilot("Pilot A");
        let b = f1_graph.add_pilot("Pilot B");
        let c = f1_graph.add_pilot("Pilot Cc");

        f1_graph.add_race_edge(a, b);
        f1_graph.add_race_edge(b, c);

        let distances_ab = dijkstra_distance(&f1_graph.graph, a, b);
        let distances_bc = dijkstra_distance(&f1_graph.graph, b, c);

        assert_eq!(distances_ab, vec![1]);
        assert_eq!(distances_bc, vec![1]);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("/Users/severancea/Documents/BU 05/DS 210/finalproject/finalcode/results.csv")?;
    let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(file);

    let race_data: Vec<RaceData> = rdr.deserialize().collect::<Result<_, _>>()?;

    let unique_pilots: HashSet<&String> = race_data.iter().map(|data| &data.driver_ref).collect();
    let mut pilots: HashMap<&String, NodeIndex> = HashMap::new();
    let mut graph = DiGraph::<&String, usize>::new(); // Change the graph type to use &String
    
    for driver_ref in unique_pilots {
        let node = graph.add_node(driver_ref);
        pilots.insert(driver_ref, node.index().try_into().unwrap());
    }
    
    let mut shared_race_counts: HashMap<(NodeIndex, NodeIndex), usize> = HashMap::new();
    
    for window in race_data.windows(2) {
        let (current, next) = (&window[0], &window[1]);
        let current_node = *pilots.get(&current.driver_ref).unwrap();
        let next_node = *pilots.get(&next.driver_ref).unwrap();

        let edge = shared_race_counts.entry((current_node, next_node)).or_insert(0);
        *edge += 1;
    }

    fn node_index_to_u32(index: NodeIndex) -> u32 {
        index.index() as u32
    }
    
    for ((source, target), &weight) in shared_race_counts.iter() {
        let source_ref = node_index_to_u32(*source);
        let target_ref = node_index_to_u32(*target);
        println!("{} -> {} [weight: {}]", source_ref, target_ref, weight);
    }    
    
    println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));

    Ok(())
}