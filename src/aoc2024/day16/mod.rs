use std::collections::{BinaryHeap, HashMap};

use crate::{
    direction::{Direction, DIRS},
    vector::{Vector, VectorMap, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    grid: VectorSet,
    size: Vector,
    start: Vector,
    end: Vector,
}

impl crate::solution::Solution<i64> for Day {
    fn meta() -> crate::solution::Meta<i64> {
        crate::solution::Meta::<i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 7036,
            answer_b: 45,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut grid = VectorSet::new(size);
        let mut start: Option<Vector> = None;
        let mut end: Option<Vector> = None;
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    continue;
                }
                let pos = Vector::new_usize(x, y);
                if c == 'S' {
                    start = Some(pos);
                } else if c == 'E' {
                    end = Some(pos);
                }
                grid.insert(pos);
            }
        }
        Self {
            raw: raw.clone(),
            grid,
            size,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn part_a(&self) -> Option<i64> {
        type Node = HashMap<Direction, (i64, Vector, Direction)>;
        let mut graph: HashMap<Vector, Node> = HashMap::new();
        let mut nodes = VectorSet::new(self.size);
        nodes.insert(self.start);
        nodes.insert(self.end);
        for pos in self.grid.iter() {
            let mut dirs = Vec::new();
            for dir in DIRS {
                if self.grid.contains(pos + dir) {
                    dirs.push(dir);
                }
            }
            if dirs.len() > 2 {
                nodes.insert(pos);
            }
        }

        for start in nodes.iter() {
            'outgoing: for outgoing in DIRS {
                if !self.grid.contains(start + outgoing) {
                    continue;
                }

                let mut pos = start + outgoing;
                let mut facing = outgoing;
                let mut score = 1;
                while !nodes.contains(pos) {
                    let left = facing.rotate_left();
                    let right = facing.rotate_right();
                    if self.grid.contains(pos + facing) {
                        pos += facing;
                        score += 1;
                        continue;
                    } else if self.grid.contains(pos + left) {
                        facing = left;
                        pos += facing;
                        score += 1001;
                        continue;
                    } else if self.grid.contains(pos + right) {
                        facing = right;
                        pos += facing;
                        score += 1001;
                        continue;
                    }
                    continue 'outgoing;
                }

                graph
                    .entry(start)
                    .or_default()
                    .insert(outgoing, (score, pos, facing));
            }
        }

        let mut heap: BinaryHeap<State> = BinaryHeap::from([State {
            score: 0,
            position: self.start,
            direction: Direction::East,
        }]);
        let mut min = i64::MAX;
        let mut visited: VectorMap<i64> = VectorMap::new(self.size);
        while let Some(State {
            score,
            position,
            direction,
        }) = heap.pop()
        {
            match visited.get(position) {
                Some(s) if s > score => continue,
                _ => {}
            }
            visited.insert(position, score);
            if position == self.end {
                if score < min {
                    min = score;
                }
                continue;
            }

            for (&dir, &node) in graph.get(&position).unwrap() {
                if !visited.contains(node.1) {
                    let s = if dir == direction { 0 } else { 1000 };
                    heap.push(State {
                        score: score + node.0 + s,
                        position: node.1,
                        direction: node.2,
                    })
                }
            }
        }
        Some(min)
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

struct State {
    score: i64,
    position: Vector,
    direction: Direction,
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Eq for State {}

impl std::cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.score.eq(&other.score)
    }
}

crate::solution::test_solution!();
