use core::fmt::Debug;
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Edge<T: Clone + Copy + Debug + PartialEq> {
    start: T,
    end: T,
    weight: i64,
}

impl<T: Clone + Copy + Debug + PartialEq> Edge<T> {
    #[allow(dead_code)]
    fn new(start: T, end: T, weight: i64) -> Self {
        Edge { start, end, weight }
    }
}

#[derive(Clone, Debug)]
pub struct Graph<T: Clone + Copy + Debug + PartialEq> {
    edges: Vec<Edge<T>>,
    nodes: HashMap<T, Vec<Edge<T>>>,
}

impl<T: Clone + Copy + Debug + PartialEq + Eq + Hash> Graph<T> {
    #[allow(dead_code)]
    fn new() -> Self {
        Graph {
            edges: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, edge: Edge<T>) {
        self.edges.push(edge.clone());
        self.nodes
            .entry(edge.start)
            .or_insert_with(Vec::new)
            .push(edge);

        self.nodes
            .entry(edge.end)
            .or_insert_with(Vec::new)
            .push(edge);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cyclic_graph() {
        let mut graph = Graph::<i32>::new();

        graph.add_edge(Edge::<i32>::new(0, 1, 1));
        graph.add_edge(Edge::<i32>::new(1, 2, 1));
        graph.add_edge(Edge::<i32>::new(2, 0, 1));

        for (_, edges) in graph.nodes.iter() {
            assert_eq!(edges.len(), 2);
        }
    }
}
