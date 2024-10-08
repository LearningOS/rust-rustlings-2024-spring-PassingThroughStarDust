/*
	graph
	This problem requires you to implement a basic graph function
*/
//

use std::collections::{HashMap, HashSet};
use std::fmt;
#[derive(Debug, Clone)]
pub struct NodeNotInGraph;
impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}
pub struct UndirectedGraph {
    adjacency_table: HashMap<String, Vec<(String, i32)>>,
}
impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjacency_table: HashMap::new(),
        }
    }
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjacency_table
    }
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjacency_table
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        match self.adjacency_table.get_mut(edge.0) {
            Some(from_node_neigbors) => {
                for (to_node, weight) in &mut *from_node_neigbors {
                    if (edge.1.to_string(), edge.2) == (to_node.clone(), *weight) {
                        return;
                    }
                }
                //  该语句必须放在这，避免self.adjacency_table.get_mut(edge.0)与
                //  self.adjacency_table.get_mut(edge.1)间因为multiple mutable reference
                //  而报错（这两个都会返回指向vector的reference，所以会冲突）。
                from_node_neigbors.push((edge.1.to_string(), edge.2));
                if let Some(from_node2_neigbors) = self.adjacency_table.get_mut(edge.1) {
                    //from_node_neigbors.push((edge.1.to_string(), edge.2));
                    from_node2_neigbors.push((edge.0.to_string(), edge.2));
                } else {
                    //from_node_neigbors.push((edge.1.to_string(), edge.2));
                    self.adjacency_table.insert(edge.1.to_string(), vec![(edge.0.to_string(), edge.2)]);
                }
            }
            None => {
                self.adjacency_table.insert(edge.0.to_string(), vec![(edge.1.to_string(), edge.2)]);
                if let Some(from_node2_neigbors) = self.adjacency_table.get_mut(edge.1) {
                    from_node2_neigbors.push((edge.0.to_string(), edge.2));
                } else {
                    self.adjacency_table.insert(edge.1.to_string(), vec![(edge.0.to_string(), edge.2)]);
                }
            }
        };
    }
}
pub trait Graph {
    fn new() -> Self;
    fn adjacency_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;
    fn adjacency_table(&self) -> &HashMap<String, Vec<(String, i32)>>;
    fn add_node(&mut self, node: &str) -> bool {
        //TODO
        match self.adjacency_table().get(node) {
            Some(_) => false,
            None => {
                self.adjacency_table_mutable().insert(node.to_owned(), Vec::new());
                true
            }
        }
    }
    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        //TODO
        //  这里应该保留空白，因为Graph分directed graph与undirected graph两种，处理方式各不相同
    }
    fn contains(&self, node: &str) -> bool {
        self.adjacency_table().get(node).is_some()
    }
    fn nodes(&self) -> HashSet<&String> {
        self.adjacency_table().keys().collect()
    }
    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbours) in self.adjacency_table() {
            for (to_node, weight) in from_node_neighbours {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}
#[cfg(test)]
mod test_undirected_graph {
    use super::Graph;
    use super::UndirectedGraph;
    #[test]
    fn test_add_edge() {
        let mut graph = UndirectedGraph::new();
        graph.add_edge(("a", "b", 5));
        graph.add_edge(("b", "c", 10));
        graph.add_edge(("c", "a", 7));
        let expected_edges = [
            (&String::from("a"), &String::from("b"), 5),
            (&String::from("b"), &String::from("a"), 5),
            (&String::from("c"), &String::from("a"), 7),
            (&String::from("a"), &String::from("c"), 7),
            (&String::from("b"), &String::from("c"), 10),
            (&String::from("c"), &String::from("b"), 10),
        ];
        for edge in expected_edges.iter() {
            assert_eq!(graph.edges().contains(edge), true);
        }
    }
}