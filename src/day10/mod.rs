#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample_a.txt");
pub const SAMPLE_B: &str = include_str!("input_sample_b.txt");
pub const ANSWER_A: i64 = 4;
pub const ANSWER_B: i64 = 4;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    start: Spot,
    map: HashMap<Spot, HashSet<Direction>>,
    size: (i64, i64),
}

type Spot = (i64, i64);

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn reverse(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Self::North,
            Direction::East => Self::West,
            Direction::West => Self::East,
        }
    }

    fn add(&self, spot: Spot) -> Spot {
        match self {
            Direction::North => (spot.0, spot.1 - 1),
            Direction::South => (spot.0, spot.1 + 1),
            Direction::East => (spot.0 + 1, spot.1),
            Direction::West => (spot.0 - 1, spot.1),
        }
    }

    fn iter() -> Vec<Direction> {
        vec![Direction::North, Direction::South, Direction::East, Direction::West]
    }
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut map = HashMap::new();
        let mut start = (0, 0);
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' { continue; }
                let spot = (x as i64, y as i64);
                let pipe = match c {
                    '|' => vec![Direction::North, Direction::South],
                    '-' => vec![Direction::East, Direction::West],
                    'L' => vec![Direction::North, Direction::East],
                    'J' => vec![Direction::North, Direction::West],
                    '7' => vec![Direction::South, Direction::West],
                    'F' => vec![Direction::South, Direction::East],
                    'S' => {
                        start = spot;
                        Direction::iter()
                    },
                    _ => panic!()
                };
                map.insert(spot, pipe.into_iter().collect());
            }
        }
        Self {
            raw: raw.clone(),
            start,
            map,
            size: (raw[0].len() as i64, raw.len() as i64)
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        for d in Direction::iter() {
            if let Some(path) = self.follow_path(self.start, d) {
                return Some(path.len() as i64 / 2);
            }
        }

        None
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut stack: Vec<(Spot, Direction)> = Vec::new();
        for dir in self.map.get(&self.start).unwrap() {
            stack.push((self.start, *dir));
        }

        let mut path: HashSet<Spot> = HashSet::new();
        for s in &stack {
            if let Some(p) = self.follow_path(s.0, s.1) {
                path = p.into_iter().collect();
                break;
            }
        }

        let mut double: HashSet<Spot> = HashSet::new();
        for pos in &path {
            let set = self.map.get(pos).unwrap();
            let new = (pos.0 * 2, pos.1 * 2);
            double.insert(new);
            if set.contains(&Direction::East) {
                double.insert((new.0 + 1, new.1));
            }
            if set.contains(&Direction::South) {
                double.insert((new.0, new.1 + 1));
            }
        }

        let mut seen: HashSet<Spot> = HashSet::new();
        let mut queue: Vec<Spot> = vec![(0, 0)];

        while let Some(pos) = queue.pop() {
            if seen.contains(&pos) { continue; }
            seen.insert(pos);

            for dir in Direction::iter() {
                let next = dir.add(pos);
                if double.contains(&next) { continue; }
                if next.0 >= 0 && next.0 < self.size.0 * 2 && next.1 >= 0 && next.1 < self.size.1 * 2 {
                    queue.insert(0, next);
                }
            }
        }

        let out = seen.iter().filter(|(x, y)| x % 2 + y % 2 == 0).count();
        Some(self.size.0 * self.size.1 - (out + path.len()) as i64)
    }

    fn follow_path(&self, pos: Spot, dir: Direction) -> Option<Vec<Spot>> {
        let next = dir.add(pos);
        if next == self.start { return Some(vec![pos]); }

        let reverse = dir.reverse();
        if let Some(set) = self.map.get(&next) {
            if set.contains(&reverse) {
                let d = set.iter().filter(|d| **d != reverse).collect::<Vec<_>>()[0];
                if let Some(mut vec) = self.follow_path(next, *d) {
                    vec.push(pos);
                    return Some(vec);
                }
            }
        }

        None
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
