use std::{
    collections::{BTreeSet, HashSet},
    rc::Rc,
};

use crate::graph::Graph;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    graph: Graph<Rc<str>, i64>,
}

impl crate::solution::Solution<i64, String> for Day {
    fn meta() -> crate::solution::Meta<i64, String> {
        crate::solution::Meta::<i64, String> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 7,
            answer_b: "co,de,ka,ta".to_string(),
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut links = HashSet::new();
        for line in raw.iter() {
            let (left, right) = line.split_once("-").unwrap();
            links.insert((left.to_string(), right.to_string()));
            links.insert((right.to_string(), left.to_string()));
        }

        let mut graph: Graph<Rc<str>, i64> = Graph::new();
        for link in links {
            graph.add_edge(&link.0.into(), &link.1.into(), 1);
        }

        Self {
            raw: raw.clone(),
            graph,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut lan = 0;
        for clique in self.graph.get_cliques(3) {
            for computer in clique {
                if computer.starts_with("t") {
                    lan += 1;
                    break;
                }
            }
        }
        Some(lan)
    }

    fn part_b(&self) -> Option<String> {
        let mut cliques: HashSet<BTreeSet<Rc<str>>> = HashSet::new();
        for node in self.graph.nodes() {
            cliques.insert(BTreeSet::from([node]));
        }

        loop {
            let mut next: HashSet<BTreeSet<Rc<str>>> = HashSet::new();
            for clique in &cliques {
                for c in self
                    .graph
                    .get_containing_cliques(&clique.iter().cloned().collect())
                {
                    next.insert(c.iter().cloned().collect());
                }
            }
            if next.is_empty() {
                break;
            }
            cliques = next.iter().cloned().collect();
        }

        let clique = cliques.iter().next().unwrap();
        let mut clique: Vec<_> = clique.iter().cloned().collect();
        clique.sort();
        Some(clique.join(","))
    }
}

crate::solution::test_solution!();
