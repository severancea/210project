// ds210 final project
// andrew severance | asev@bu.edu

// graph.rs includes all of the functions needed to build the graph, which will in turn tell us about who
// has competed the most other drivers, as well as which two drivers have competed against each other the most.
use petgraph::graph::{Graph, NodeIndex};
use petgraph::algo::dijkstra;

pub struct F1Graph {
    pub graph: Graph<String, ()>,
}

impl F1Graph {
    pub fn new() -> Self {
        F1Graph { graph: Graph::new() }
    }

    pub fn add_pilot(&mut self, driver_ref: &str) -> NodeIndex<u32> {
        let node_index = self.graph.add_node(driver_ref.to_string());
        node_index
    }

    pub fn add_race_edge(&mut self, pilot1: NodeIndex<u32>, pilot2: NodeIndex<u32>) {
        self.graph.add_edge(pilot1, pilot2, ());
    }

    pub fn find_driver_node(&self, driver_name: &str) -> Option<NodeIndex<u32>> {
        self.graph
            .node_indices()
            .find(|&node| self.graph[node] == driver_name)
    }

    pub fn find_nodes_with_distance(
        &self,
        start_name: &str,
        end_name: &str,
    ) -> Option<(NodeIndex<u32>, NodeIndex<u32>, usize)> {
        if let Some(start_node) = self.find_driver_node(start_name) {
            if let Some(end_node) = self.find_driver_node(end_name) {
                let distances = dijkstra(&self.graph, start_node, None, |_| 1);
                if let Some(distance) = distances.get(&end_node) {
                    return Some((start_node, end_node, *distance));
                }
            }
        }
        None
    }
}