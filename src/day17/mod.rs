#![allow(dead_code)]

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 102;
pub const ANSWER_B: i64 = 94;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    blocks: HashMap<Pos, i64>,
    size: (i64, i64),
}

type Pos = (i64, i64);
type Graph<V, E> = BTreeMap<V, BTreeMap<V, E>>;
const DIRS: [Pos; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut blocks = HashMap::new();
        for (row, line) in raw.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                blocks.insert((row as i64, col as i64), ch as i64 - b'0' as i64);
            }
        }

        Self {
            raw: raw.clone(),
            blocks,
            size: (raw.len() as i64, raw[0].len() as i64),
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let graph = self.build_graph(|i| i < 3);
        Some(self.dijkstra(&graph))
    }

    pub fn part_b(&self) -> Option<i64> {
        let graph = self.build_graph(|i| i >= 3);
        Some(self.dijkstra(&graph))
    }

    fn build_graph(&self, filter: fn(i64) -> bool) -> Graph<Pos, i64> {
        let mut graph = Graph::new();

        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                let mut edges = BTreeMap::new();
                for dir in DIRS {
                    let mut pos = (row, col);
                    let mut weight = 0;
                    for i in 0..10 {
                        pos = (pos.0 + dir.0, pos.1 + dir.1);
                        match self.blocks.get(&pos) {
                            Some(w) => weight += w,
                            None => break,
                        }

                        if filter(i) {
                            edges.insert(pos, weight);
                        }
                    }
                }
                graph.insert((row, col), edges);
            }
        }

        graph
    }

    fn dijkstra(&self, graph: &Graph<Pos, i64>) -> i64 {
        let start = (0, 0);
        let end = (self.size.0 - 1, self.size.1 - 1);
        let mut distances = HashMap::new();
        let mut priority = BinaryHeap::new();

        // start is the special case that doesn't have a predecessor
        distances.insert((start, None), None);

        for (node, weight) in &graph[&start] {
            let is_horz = start.1 == node.1;
            distances.insert((*node, Some(is_horz)), Some((start, *weight)));
            priority.push(Reverse((*weight, *node, start, is_horz)));
        }

        while let Some(Reverse((distance, node, prev, was_horz))) = priority.pop() {
            match distances[&(node, Some(was_horz))] {
                // what we popped is what is in distances, we'll compute it
                Some((p, d)) if p == prev && d == distance => {}
                // otherwise it's not interesting
                _ => continue,
            }

            for (next, weight) in &graph[&node] {
                let is_horz = node.1 == next.1;
                if is_horz == was_horz {
                    continue;
                }

                match distances.get(&(*next, Some(is_horz))) {
                    // if distances[next] is a lower dist than the alternative one, we do nothing
                    Some(Some((_, dist_next))) if *dist_next <= distance + *weight => {}
                    // if distances[next] is None then next is start and so the distance won't be changed, it won't be added again in priority
                    Some(None) => {}
                    // the new path is shorter, either next was not in distances or it was farther
                    _ => {
                        distances.insert((*next, Some(is_horz)), Some((node, *weight + distance)));
                        priority.push(Reverse((*weight + distance, *next, node, is_horz)));
                    }
                }
            }
        }

        let mut min = i64::MAX;
        min = min.min(distances.get(&(end, Some(true))).unwrap().unwrap().1);
        min = min.min(distances.get(&(end, Some(false))).unwrap().unwrap().1);
        min
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE_A));
        assert_eq!(solution.part_a().unwrap_or(0), ANSWER_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE_B));
        assert_eq!(solution.part_b().unwrap_or(0), ANSWER_B);
    }
}
