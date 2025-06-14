use crate::graph::Graph;
use crate::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    graph: Graph<Rc<str>, i64>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 54,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut graph = Graph::new();
        for line in raw.iter() {
            let (left, right) = line.split_once(": ").unwrap();
            let left = left.into();
            for right in right.split_whitespace() {
                graph.add_edge(&left, &right.into(), 1);
            }
        }

        Self {
            raw: raw.clone(),
            graph,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut graph = self.graph.clone();
        let bridges = graph.minimum_cut().unwrap();
        graph.remove_edge(&bridges[0].0, &bridges[0].1);
        graph.remove_edge(&bridges[1].0, &bridges[1].1);
        graph.remove_edge(&bridges[2].0, &bridges[2].1);
        let a = graph.size_of_group(&bridges[0].0);
        let b = graph.size_of_group(&bridges[0].1);
        Some((a * b) as i64)
    }

    fn part_b(&self) -> Option<i64> {
        Some(0)
    }
}

crate::solution::test_solution!();
