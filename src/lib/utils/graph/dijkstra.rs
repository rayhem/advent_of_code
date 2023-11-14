use std::collections;

type Cost = i32;

/// Minimal Representation of a node in a shortest-path search.
pub trait Node
where
    Self: Sized,
{
    /// Called by the search algorithm to determine if `&self` is the
    /// destination node. This allows for very abstract representations of a
    /// node/graph where stopping criteria may not simply be a node's name
    fn is_destination(&self) -> bool;

    fn neighbors(&self) -> Vec<(Cost, Self)>;
}

#[derive(Debug)]
pub struct ShortestPathEntry<N> {
    cost: Cost,
    node: N,
}

impl<N> ShortestPathEntry<N> {
    fn new(node: N) -> Self {
        ShortestPathEntry { cost: 0, node }
    }
}

impl<N> From<(Cost, N)> for ShortestPathEntry<N> {
    fn from((cost, node): (Cost, N)) -> Self {
        ShortestPathEntry { cost, node }
    }
}

impl<N> PartialOrd for ShortestPathEntry<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&self.cost))
    }
}

impl<N> Ord for ShortestPathEntry<N> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<N> PartialEq for ShortestPathEntry<N> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<N> Eq for ShortestPathEntry<N> {}

#[allow(dead_code)]
pub fn search<T: Node>(begin: T) -> Cost {
    let mut queue = collections::BinaryHeap::<ShortestPathEntry<T>>::new();

    queue.push(ShortestPathEntry::new(begin));

    while let Some(ShortestPathEntry { cost, node }) = queue.pop() {
        if node.is_destination() {
            return cost;
        }

        let neighbors: Vec<_> = node
            .neighbors()
            .into_iter()
            .map(|(weight, neighbor)| ShortestPathEntry::from((cost + weight, neighbor)))
            .collect();

        queue.extend(neighbors)
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Finds the shortest path from 0 to 10 amongst a trinary tree of integers
    /// such that `i` connects to `3 * i + 1` (weight = 1), `3 * i + 2` (weight
    /// = 2), and `3 * i + 3` (weight = 3):
    /// 0
    /// ├── 1
    /// │   ├── 4
    /// │   ├── 5
    /// │   └── 6
    /// ├── 2
    /// │   ├── 7
    /// │   ├── 8
    /// │   └── 9
    /// └── 3
    ///     ├── 10
    ///     ├── 11
    ///     └── 12
    /// where first connection out of a node (e.g. 0->1) has weight 1, the
    /// second has weight 2, and the third has weight 3. So, weight(0->3) +
    /// weight(3->10) = 3 + 1 = 4 which is the shortest (and only) path from 0
    /// to 10
    fn trinary_tree() {
        impl super::Node for i32 {
            fn is_destination(&self) -> bool {
                *self == 10
            }

            fn neighbors(&self) -> Vec<(i32, Self)> {
                ((3 * self + 1)..=(3 * self + 3))
                    .enumerate()
                    .map(|(i, n)| ((i + 1) as Cost, n))
                    .collect()
            }
        }

        assert_eq!(search(0), 4);
    }
}
