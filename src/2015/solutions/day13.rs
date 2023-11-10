use itertools::Itertools;
use utils::graph::{self, Edge};
use utils::solution::Solution;
pub struct Day13 {}

impl Solution for Day13 {
    fn part_one(&self, input: &str) -> Option<String> {
        let (_, happiness) = graph::EdgeList::from(input.lines().map(to_edge))
            .shortest_cyclic_tour_by(pairwise_happiness);

        Some(happiness.abs().to_string())
    }

    fn part_two(&self, input: &str) -> Option<String> {
        let mut edges = input.lines().map(to_edge).collect::<Vec<_>>();
        let nodes = edges
            .iter()
            .flat_map(|edge| [edge.start.clone(), edge.end.clone()].into_iter())
            .unique()
            .collect::<Vec<_>>();

        edges.extend(nodes.into_iter().flat_map(|name| {
            [
                Edge::with_data("me".to_string(), name.clone(), 0),
                Edge::with_data(name, "me".to_string(), 0),
            ]
            .into_iter()
        }));

        let (_, happiness) =
            graph::EdgeList::from(edges).shortest_cyclic_tour_by(pairwise_happiness);

        Some(happiness.abs().to_string())
    }
}

fn to_edge(s: &str) -> graph::Edge<String, i32> {
    let mut tokens = s.split_ascii_whitespace();

    let from = tokens.next().unwrap().to_string();
    let sign = match tokens.nth(1).unwrap() {
        "gain" => 1,
        "lose" => -1,
        _ => unreachable!(),
    };
    let value = tokens.next().unwrap().parse::<i32>().unwrap();
    let to = tokens.last().unwrap().trim_end_matches('.').to_string();

    graph::Edge::with_data(from, to, sign * value)
}

fn pairwise_happiness(fwd: Option<i32>, bkwd: Option<i32>) -> Option<i32> {
    match (fwd, bkwd) {
        (Some(f), Some(b)) => Some(-(f + b)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        const INPUT: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";
        let graph = graph::EdgeList::from(INPUT.lines().map(to_edge));

        let (_, happiness) = graph.shortest_cyclic_tour_by(pairwise_happiness);
        assert_eq!(happiness.abs(), 330);
    }
}

utils::verify!(Day13, utils::my_input!("2015", "13"), "709", "668");
