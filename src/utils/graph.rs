use core::fmt::Debug;
use std::collections::HashMap;
use std::hash::Hash;

use itertools::{FoldWhile, Itertools};

/// Details an edge in a graph between nodes `a` and `b`. The `data` field
/// defaults to the zero-sized unit type, `()`, which is suitable for undirected
/// graphs. The generic flexibility in `data` allows for representations of
/// weighted and directed graphs or named edges.
#[derive(Clone, Debug, PartialEq)]
pub struct Edge<T, U = ()> {
    pub start: T,
    pub end: T,
    pub data: U,
}

impl<T> Edge<T> {
    /// Construct an [`Edge`], leaving the `data` field empty
    pub fn new(start: T, end: T) -> Self {
        Self {
            start,
            end,
            data: (),
        }
    }
}

impl<T, U> Edge<T, U> {
    /// Construct an [`Edge`] with data
    pub fn with_data(start: T, end: T, data: U) -> Self {
        Self { start, end, data }
    }
}

#[derive(Clone, Debug)]
pub struct Graph<T, U> {
    pub edges: Vec<Edge<T, U>>,

    /// Association of a node name with the [`Edge`]s connected to that node.
    /// Note: [`Edge`]s may be undirected, incoming, outgoing, or symmetric
    /// depending on context.
    pub nodes: HashMap<T, Vec<usize>>,
}

impl<T: Clone + Debug + Eq + Hash, U: Clone> Graph<T, U> {
    pub fn new() -> Self {
        Graph {
            edges: Vec::new(),
            nodes: HashMap::new(),
        }
    }

    /// Add an edge to the graph
    pub fn add_edge(&mut self, edge: Edge<T, U>) {
        let edge_index = self.edges.len();
        self.edges.push(edge.clone());

        self.nodes.entry(edge.start).or_default().push(edge_index);
        self.nodes.entry(edge.end).or_default().push(edge_index);
    }
}

impl<T: Clone + Debug + Eq + Hash, U: Clone, V: IntoIterator<Item = Edge<T, U>>> From<V>
    for Graph<T, U>
{
    /// Construct a `[Graph]` from an iterator over edges
    fn from(value: V) -> Self {
        let mut graph = Graph::new();

        for item in value.into_iter() {
            graph.add_edge(item);
        }

        graph
    }
}

impl<T: Clone + Debug + Eq + Hash, U: Clone> Default for Graph<T, U> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct EdgeList<T, U> {
    nodes: Vec<T>,
    edges: HashMap<(usize, usize), U>,
}

impl<T, U, V> From<V> for EdgeList<T, U>
where
    T: Clone + Eq + Hash,
    U: Clone,
    V: Clone + IntoIterator<Item = Edge<T, U>>,
{
    fn from(value: V) -> Self {
        let nodes = value
            .clone()
            .into_iter()
            .flat_map(|edge| [edge.start, edge.end].into_iter())
            .unique()
            .collect::<Vec<_>>();

        let edges = value
            .into_iter()
            .map(|edge| {
                (
                    (
                        nodes.iter().position(|x| *x == edge.start).unwrap(),
                        nodes.iter().position(|x| *x == edge.end).unwrap(),
                    ),
                    edge.data,
                )
            })
            .collect();

        Self { nodes, edges }
    }
}

impl<T, Weight: Copy + Ord + Default + std::ops::Add<Output = Weight>> EdgeList<T, Weight> {
    fn shortest_tour_impl<V: Iterator<Item = Vec<usize>>>(
        &self,
        dist: fn(Weight, Weight) -> Weight,
        tours: V,
    ) -> (Vec<usize>, Weight) {
        tours
            .flat_map(|tour| {
                tour.iter()
                    .tuple_windows()
                    .fold_while(Some(Weight::default()), |acc, (&start, &end)| match acc {
                        None => FoldWhile::Done(None),
                        Some(n) => {
                            FoldWhile::Continue(self.edges.get(&(start, end)).and_then(|forward| {
                                self.edges
                                    .get(&(end, start))
                                    .map(|backward| n + dist(*forward, *backward))
                            }))
                        }
                    })
                    .into_inner()
                    .map(|tour_length| (tour, tour_length))
            })
            .min_by_key(|(_, tour_length)| *tour_length)
            .unwrap()
    }

    /// Finds the shortest cyclic tour (i.e. start == end) through the graph,
    /// evaluating `dist` for every edge. The tour distance is then the
    /// accumulation of `dist` over all edges. This accommodates the general
    /// case for directed graphs where the forward distance does not equal the
    /// reverse distance.
    ///
    /// Assumes the tour starts and ends at nodes[0].
    pub fn shortest_cyclic_tour_by(
        &self,
        dist: fn(forward_weight: Weight, backward_weight: Weight) -> Weight,
    ) -> (Vec<usize>, Weight) {
        let indices = (1..self.nodes.len())
            .permutations(self.nodes.len() - 1)
            .map(|v| {
                std::iter::once(0)
                    .chain(v)
                    .chain(std::iter::once(0))
                    .collect()
            });
        self.shortest_tour_impl(dist, indices)
    }

    /// Finds the shortest acyclic tour (i.e. start != end) through the graph,
    /// evaluating `dist` for every edge. The tour distance is then the
    /// accumulation of `dist` over all edges. This accommodates the general
    /// case for directed graphs where the forward distance does not equal the
    /// reverse distance.
    pub fn shortest_acyclic_tour_by(
        &self,
        dist: fn(Weight, Weight) -> Weight,
    ) -> (Vec<usize>, Weight) {
        let indices = (0..self.nodes.len()).permutations(self.nodes.len());
        self.shortest_tour_impl(dist, indices)
    }

    /// Finds the shortest cyclic tour of an undirected graph by accumulating
    /// the forward distance for each pair of nodes
    pub fn shortest_cyclic_tour(&self) -> (Vec<usize>, Weight) {
        self.shortest_cyclic_tour_by(|p, _| p)
    }

    /// Finds the shortest acyclic tour of an undirected graph by accumulating
    /// the forward distance for each pair of nodes
    pub fn shortest_acyclic_tour(&self) -> (Vec<usize>, Weight) {
        self.shortest_acyclic_tour_by(|p, _| p)
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
