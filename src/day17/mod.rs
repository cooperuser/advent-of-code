#![allow(dead_code)]

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};

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
const DIRS: [Dir; 4] = [Dir::North, Dir::South, Dir::East, Dir::West];

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn add(&self, pos: Pos) -> Pos {
        match self {
            Dir::North => (pos.0 - 1, pos.1),
            Dir::South => (pos.0 + 1, pos.1),
            Dir::East => (pos.0, pos.1 + 1),
            Dir::West => (pos.0, pos.1 - 1),
        }
    }

    fn sub(start: Pos, end: Pos) -> Option<Self> {
        let pos = (end.0 - start.0, end.1 - start.1);
        if pos.0 < 0 && pos.1 == 0 {
            Some(Self::North)
        } else if pos.0 > 0 && pos.1 == 0 {
            Some(Self::South)
        } else if pos.0 == 0 && pos.1 < 0 {
            Some(Self::West)
        } else if pos.0 == 0 && pos.1 > 0 {
            Some(Self::East)
        } else {
            None
        }
    }

    fn rev(&self) -> Self {
        match self {
            Dir::North => Dir::South,
            Dir::South => Dir::North,
            Dir::East => Dir::West,
            Dir::West => Dir::East,
        }
    }
}

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
        let mut graph = BTreeMap::new();

        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                let mut edges = BTreeMap::new();
                for dir in DIRS {
                    let mut pos = (row, col);
                    let mut weight = 0;
                    for i in 0..10 {
                        pos = dir.add(pos);
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
            let dir = Dir::sub(start, *node);
            distances.insert((*node, dir), Some((start, *weight)));
            priority.push(Reverse((*weight, *node, start, dir.unwrap())));
        }

        while let Some(Reverse((distance, node, prev, dir))) = priority.pop() {
            match distances[&(node, Some(dir))] {
                // what we popped is what is in distances, we'll compute it
                Some((p, d)) if p == prev && d == distance => {}
                // otherwise it's not interesting
                _ => continue,
            }

            for (next, weight) in &graph[&node] {
                let last_dir = Dir::sub(prev, node).unwrap();
                let dir = Dir::sub(node, *next).unwrap();
                // if prev == *next || Dir::sub(prev, *next).is_some() {
                if dir == last_dir || dir == last_dir.rev() {
                    continue;
                }

                match distances.get(&(*next, Some(dir))) {
                    // if distances[next] is a lower dist than the alternative one, we do nothing
                    Some(Some((_, dist_next))) if *dist_next <= distance + *weight => {}
                    // if distances[next] is None then next is start and so the distance won't be changed, it won't be added again in priority
                    Some(None) => {}
                    // the new path is shorter, either next was not in distances or it was farther
                    _ => {
                        distances.insert((*next, Some(dir)), Some((node, *weight + distance)));
                        priority.push(Reverse((*weight + distance, *next, node, dir)));
                    }
                }
            }
        }

        let mut min = i64::MAX;
        for dir in DIRS {
            if let Some(test) = distances.get(&(end, Some(dir))) {
                min = min.min(test.unwrap().1);
            }
        }
        min
    }

    fn dump_path(&self, distances: &HashMap<Pos, Option<(Pos, i64)>>, node: Pos) {
        let mut node = node;
        let mut path = HashSet::new();
        while node != (0, 0) {
            if let Some(Some((prev, _))) = distances.get(&node) {
                path.insert(node);
                node = *prev;
            }
        }

        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                let c = if path.contains(&(row, col)) {
                    let a = (row, col);
                    let b = distances.get(&a).unwrap().unwrap().0;
                    let dir = Dir::sub(a, b).unwrap();
                    match dir {
                        Dir::North => '^',
                        Dir::South => 'v',
                        Dir::East => '>',
                        Dir::West => '<',
                    }
                } else {
                    '.'
                };
                // print!("{}{}", self.blocks.get(&(row, col)).unwrap(), c);
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    fn dump_directions(&self, distances: &HashMap<Pos, Option<(Pos, i64)>>) {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                let node = (row, col);
                // if let Some((prev, _)) = distances.get(&node).unwrap() {
                if let Some(Some((prev, _))) = distances.get(&node) {
                    let dir = Dir::sub(*prev, node).unwrap();
                    let c = match dir.rev() {
                        Dir::North => '^',
                        Dir::South => 'v',
                        Dir::East => '>',
                        Dir::West => '<',
                    };
                    print!("{}", c);
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!();
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
