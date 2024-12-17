use crate::graph::Graph;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    graph: Graph<String, ()>,
}

impl crate::solution::Solution<i64> for Day {
    fn meta() -> crate::solution::Meta<i64> {
        crate::solution::Meta::<i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 54,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let mut graph = Graph::new();
        for line in raw.iter() {
            let (left, right) = line.split_once(": ").unwrap();
            let left = left.to_string();
            for right in right.split_whitespace() {
                graph.add_edge(&left, &right.to_string(), &());
            }
        }

        Self {
            raw: raw.clone(),
            graph,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let graph = self.graph.clone();
        println!("{:?}", graph);
        None
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

crate::solution::test_solution!();
