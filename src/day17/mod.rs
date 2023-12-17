use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap, HashSet};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 102;
pub const ANSWER_B: i64 = 0;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    blocks: HashMap<Pos, i64>,
    graph: BTreeMap<Pos, BTreeMap<Pos, i64>>,
    size: (i64, i64),
}

type Pos = (i64, i64);
const DIRS: [Dir; 4] = [Dir::North, Dir::South, Dir::East, Dir::West];

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
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

    fn sub(start: Pos, end: Pos) -> Self {
        match (end.0 - start.0, end.1 - start.1) {
            (-1, 0) => Self::North,
            (1, 0) => Self::South,
            (0, 1) => Self::East,
            (0, -1) => Self::West,
            _ => panic!("not adjacent"),
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
        let mut graph = BTreeMap::new();
        for (row, line) in raw.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                blocks.insert((row as i64, col as i64), ch.to_string().parse().unwrap());
            }
        }

        let size = (raw.len() as i64, raw[0].len() as i64);
        for row in 0..size.0 {
            for col in 0..size.1 {
                let start = (row, col);
                let mut edges = BTreeMap::new();
                for dir in DIRS {
                    let end = dir.add(start);
                    if end.0 < 0 || end.0 >= size.0 || end.1 < 0 || end.1 >= size.1 {
                        continue;
                    }
                    edges.insert(end, *blocks.get(&end).unwrap());
                }
                graph.insert(start, edges);
            }
        }

        Self {
            raw: raw.clone(),
            blocks,
            graph,
            size,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let start = (0, 0);
        let end = (self.size.0 - 1, self.size.1 - 1);
        let mut distances = HashMap::new();
        let mut priority = BinaryHeap::new();
        let mut directions = HashMap::new();
        distances.insert(start, None);

        for (node, weight) in self.graph.get(&start).unwrap() {
            distances.insert(*node, Some((start, *weight)));
            priority.push(Reverse((*weight, *node, start)));
            directions.insert(*node, (Dir::sub(start, *node), 1));
        }

        while let Some(Reverse((distance, node, prev))) = priority.pop() {
            match distances[&node] {
                Some((p, d)) if p == prev && d == distance => {}
                _ => continue,
            }
            self.dump_map(&directions, node);

            for (next, weight) in self.graph.get(&node).unwrap() {
                match distances.get(next) {
                    Some(Some((_, dist_next))) if *dist_next <= distance + *weight => {}
                    Some(None) => {}
                    _ => {
                        let dir = Dir::sub(node, *next);
                        if let Some((last, count)) = directions.get(&node) {
                            if dir == *last && *count > 2 {
                                continue;
                            } else if dir == *last {
                                directions.insert(*next, (dir, count + 1));
                            } else {
                                directions.insert(*next, (dir, 1));
                            }
                        }
                        distances.insert(*next, Some((node, *weight + distance)));
                        priority.push(Reverse((*weight + distance, *next, node)));
                    }
                }
            }
        }

        println!("{:?}", distances.get(&end));
        // println!("{:?}", distances);
        self.dump_map(&directions, end);
        // self.dump_distances(&distances);

        Some(distances.get(&end).unwrap().unwrap().1)
    }

    pub fn part_b(&self) -> Option<i64> {
        None
    }

    fn dump_map(&self, directions: &HashMap<Pos, (Dir, i64)>, node: Pos) {
        let mut node = node;
        let mut path = HashSet::new();
        while node != (0, 0) {
            if let Some((dir, _)) = directions.get(&node) {
                path.insert(node);
                node = dir.rev().add(node);
            }
        }

        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                let c = if path.contains(&(row, col)) {
                    match directions.get(&(row, col)) {
                        Some((Dir::North, _)) => '^',
                        Some((Dir::South, _)) => 'v',
                        Some((Dir::East, _)) => '>',
                        Some((Dir::West, _)) => '<',
                        None => '.',
                    }
                } else {
                    '.'
                };
                print!("{}", c);
            }
            println!();
        }
        println!();
    }

    fn dump_distances(&self, distances: &HashMap<Pos, Option<(Pos, i64)>>) {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                let node = (row, col);
                if let Some((prev, _)) = distances.get(&node).unwrap() {
                    let dir = Dir::sub(*prev, node);
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
