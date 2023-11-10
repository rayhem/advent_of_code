use core::fmt::Debug;
use std::collections::HashMap;
use std::hash::Hash;

/// Details an edge in a graph between nodes `a` and `b`. The `data` field
/// defaults to the zero-sized unit type, `()`, which is suitable for undirected
/// graphs. The generic flexibility in `data` allows for representations of
/// weighted and/or directed graphs or even things like named edges.
#[derive(Clone, Debug, PartialEq)]
pub struct Edge<T, U = ()> {
    pub a: T,
    pub b: T,
    pub data: U,
}

impl<T> Edge<T> {
    /// Construct an [`Edge`], leaving the `data` field empty
    pub fn new(a: T, b: T) -> Self {
        Self { a, b, data: () }
    }
}

impl<T, U> Edge<T, U> {
    /// Construct an [`Edge`] with data
    pub fn with_data(a: T, b: T, data: U) -> Self {
        Self { a, b, data }
    }
}

#[derive(Clone, Debug)]
pub struct Graph<T> {
    pub edges: Vec<Edge<T>>,

    /// Association of a node name with the [`Edge`]s connected to that node.
    /// Note: [`Edge`]s may be undirected, incoming, outgoing, or symmetric
    /// depending on context.
    pub nodes: HashMap<T, Vec<usize>>,
}

impl<T: Clone + Debug + Eq + Hash> Graph<T> {
    pub fn new() -> Self {
        Graph {
            edges: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, edge: Edge<T>) {
        let edge_index = self.edges.len();
        self.edges.push(edge.clone());

        self.nodes
            .entry(edge.a)
            .or_insert_with(Vec::new)
            .push(edge_index);

        self.nodes
            .entry(edge.b)
            .or_insert_with(Vec::new)
            .push(edge_index);
    }
}

impl<T: Clone + Debug + Eq + Hash, U: IntoIterator<Item = Edge<T>>> From<U> for Graph<T> {
    /// Construct a `[Graph]` from an iterator over edges
    fn from(value: U) -> Self {
        let mut graph = Graph::new();

        for item in value.into_iter() {
            graph.add_edge(item);
        }

        graph
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_edge() {
        let mut graph = Graph::new();

        graph.add_edge(Edge::new(0, 1));

        assert_eq!(graph.edges.len(), 1);
        assert_eq!(graph.nodes.get(&0), Some(&vec![0]));
        assert_eq!(graph.nodes.get(&1), Some(&vec![0]));
    }

    #[test]
    fn add_cycle() {
        let edges = vec![Edge::new(0, 1), Edge::new(1, 2), Edge::new(2, 0)];

        let graph = Graph::from(edges);

        assert_eq!(graph.edges.len(), 3);
        assert_eq!(graph.nodes.get(&0), Some(&vec![0, 2]));
        assert_eq!(graph.nodes.get(&1), Some(&vec![0, 1]));
        assert_eq!(graph.nodes.get(&2), Some(&vec![1, 2]));
    }
}
