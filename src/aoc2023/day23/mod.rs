use crate::prelude::*;
use std::collections::{BinaryHeap, HashSet, VecDeque};

use crate::{
    direction::{Direction, DIRS},
    vector::{Vector, VectorMap, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    grid: VectorMap<Tile>,
    start: Vector,
    size: Vector,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Forest,
    Path,
    Slope(Direction),
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 94,
            answer_b: 154,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut grid = VectorMap::new(size);
        let mut start = None;
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Vector::new_usize(x, y);
                if y == 0 && c == '.' {
                    start = Some(pos);
                }
                grid.insert(pos, c.to_string().parse().unwrap());
            }
        }

        Self {
            raw: raw.clone(),
            grid,
            start: start.unwrap(),
            size,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut heap: BinaryHeap<State> = BinaryHeap::from([State {
            distance: 0,
            position: self.start,
            direction: Some(Direction::South),
            visited: VectorSet::new(self.size),
        }]);
        let mut seen: VectorMap<i64> = VectorMap::new(self.size);
        let mut max = 0;
        while let Some(State {
            distance,
            position: pos,
            direction: last_dir,
            visited: mut set,
        }) = heap.pop()
        {
            if !set.insert(pos).unwrap() {
                continue;
            }
            if let Some(last) = seen.get(pos) {
                if last > distance {
                    continue;
                }
            }
            seen.insert(pos, distance);

            if pos.y == self.size.y - 1 && distance > max {
                max = distance;
                continue;
            }

            let mut next: Vec<Direction> = Vec::new();
            match self.grid.get(pos) {
                Some(Tile::Path) => {
                    for dir in DIRS {
                        if Some(dir.flip()) == last_dir {
                            continue;
                        }
                        match self.grid.get(pos + dir) {
                            None | Some(Tile::Forest) => {}
                            _ => next.push(dir),
                        }
                    }
                }
                Some(Tile::Slope(slope)) => {
                    next.push(slope);
                }
                _ => {}
            }

            if next.len() == 1 {
                heap.push(State {
                    distance: distance + 1,
                    position: pos + next[0],
                    direction: Some(next[0]),
                    visited: set,
                });
                continue;
            }
            for dir in next {
                heap.push(State {
                    distance: distance + 1,
                    position: pos + dir,
                    direction: Some(dir),
                    visited: set.clone(),
                });
            }
        }
        Some(max)
    }

    fn part_b(&self) -> Option<i64> {
        let mut graph: VectorMap<HashSet<(i64, Vector)>> = VectorMap::new(self.size);
        let mut deque: VecDeque<Vector> = VecDeque::from([self.start]);
        while let Some(start) = deque.pop_front() {
            if graph.contains(start) {
                continue;
            }
            let mut paths: HashSet<(i64, Vector)> = HashSet::new();
            let mut path: VecDeque<(i64, Vector, Option<Direction>)> = VecDeque::new();
            for dir in DIRS {
                match self.grid.get(start + dir) {
                    None | Some(Tile::Forest) => {}
                    _ => path.push_back((0, start + dir, Some(dir))),
                }
            }
            while let Some((dist, pos, last_dir)) = path.pop_front() {
                let mut next: Vec<Direction> = Vec::new();
                for dir in DIRS {
                    if Some(dir.flip()) == last_dir {
                        continue;
                    }
                    match self.grid.get(pos + dir) {
                        None | Some(Tile::Forest) => {}
                        _ => {
                            next.push(dir);
                            if (pos + dir).y == self.size.y - 1 {
                                paths.insert((dist + 1, pos + dir));
                                continue;
                            }
                        }
                    }
                }
                if next.len() == 1 {
                    path.push_back((dist + 1, pos + next[0], Some(next[0])));
                } else if !next.is_empty() {
                    paths.insert((dist + 1, pos));
                    deque.push_back(pos);
                }
            }
            graph.insert(start, paths);
        }

        let mut heap: BinaryHeap<State> = BinaryHeap::from([State {
            distance: 0,
            position: self.start,
            direction: None,
            visited: VectorSet::new(self.size),
        }]);
        let mut max = 0;
        while let Some(State {
            distance,
            position: pos,
            direction: _,
            visited: mut set,
        }) = heap.pop()
        {
            if !set.insert(pos).unwrap() {
                continue;
            }

            if pos.y == self.size.y - 1 {
                if distance > max {
                    max = distance;
                    println!("{max}");
                }
                continue;
            }

            let mut next: Vec<(i64, Vector)> = Vec::new();
            for node in graph.get(pos).unwrap() {
                if !set.contains(node.1) {
                    next.push(node);
                }
            }

            if next.len() == 1 {
                heap.push(State {
                    distance: distance + next[0].0,
                    position: next[0].1,
                    direction: None,
                    visited: set,
                });
                continue;
            }
            for dir in next {
                heap.push(State {
                    distance: distance + dir.0,
                    position: dir.1,
                    direction: None,
                    visited: set.clone(),
                });
            }
        }

        Some(max + 1)
    }
}

impl std::str::FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Forest),
            "." => Ok(Self::Path),
            "^" => Ok(Self::Slope(Direction::North)),
            "v" => Ok(Self::Slope(Direction::South)),
            ">" => Ok(Self::Slope(Direction::East)),
            "<" => Ok(Self::Slope(Direction::West)),
            _ => Err(()),
        }
    }
}

struct State {
    distance: i64,
    position: Vector,
    direction: Option<Direction>,
    visited: VectorSet,
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance).reverse()
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.distance.cmp(&other.distance))
    }
}

impl std::cmp::Eq for State {}

impl std::cmp::PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

crate::solution::test_solution!();
