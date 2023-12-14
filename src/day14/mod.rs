#![allow(dead_code)]

use std::collections::{HashMap, BTreeSet};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 136;
pub const ANSWER_B: i64 = 64;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    platform: Platform,
}

#[derive(Default, Clone)]
struct Platform {
    rocks: HashMap<(i64, i64), Rock>,
    size: (i64, i64),
}

#[derive(Clone, PartialEq)]
enum Rock {
    Square,
    Round,
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Platform {
    fn tilt(&mut self, dir: Direction) {
        let down: Vec<_> = (0..self.size.0).collect();
        let up: Vec<_> = (0..self.size.0).rev().collect();
        let left: Vec<_> = (0..self.size.1).rev().collect();
        let right: Vec<_> = (0..self.size.1).collect();

        let (rows, cols, dr, dc) = match dir {
            Direction::North => (down, right, -1, 0),
            Direction::South => (up, left, 1, 0),
            Direction::East => (down, left, 0, 1),
            Direction::West => (up, right, 0, -1),
        };

        let mut rocks = HashMap::new();
        for &row in &rows {
            for &col in &cols {
                let spot = (row, col);
                match self.rocks.get(&spot) {
                    Some(Rock::Square) => { rocks.insert(spot, Rock::Square); },
                    Some(Rock::Round) => {
                        let mut r = row;
                        let mut c = col;
                        while r + dr >= 0 && c + dc >= 0 && r + dr < self.size.0 && c + dc < self.size.1 {
                            if rocks.contains_key(&(r + dr, c + dc)) {
                                break;
                            }
                            r += dr;
                            c += dc;
                        }
                        rocks.insert((r, c), Rock::Round);
                    },
                    None => ()
                }
            }
        }
        self.rocks = rocks;
    }

    fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    fn serialize(&self) -> BTreeSet<(i64, i64)> {
        self.rocks
            .iter()
            .filter(|(_, rock)| **rock == Rock::Round)
            .map(|(pos, _)| *pos)
            .collect()
    }

    fn load_north(&self) -> i64 {
        let mut load = 0;
        for (pos, rock) in &self.rocks {
            if *rock == Rock::Round {
                load += self.size.0 - pos.0;
            }
        }
        load
    }
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut rocks = HashMap::new();
        for (row, line) in raw.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                let pos = (row as i64, col as i64);
                match ch {
                    '#' => {
                        rocks.insert(pos, Rock::Square);
                    },
                    'O' => {
                        rocks.insert(pos, Rock::Round);
                    }
                    _ => ()
                }
            }
        }
        Self {
            raw: raw.clone(),
            platform: Platform {
                rocks,
                size: (raw.len() as i64, raw[0].len() as i64),
            }
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut platform = self.platform.clone();
        platform.tilt(Direction::North);
        Some(platform.load_north())
    }

    pub fn part_b(&self) -> Option<i64> {
        let count = 1_000_000_000;
        let mut platform = self.platform.clone();
        let mut seen = HashMap::new();

        let mut i = 0i128;
        let (start, end) = loop {
            let serial = platform.serialize();
            if let Some(previous) = seen.get(&serial) {
                break (previous, i);
            }
            seen.insert(serial, i);
            platform.cycle();
            i += 1;
        };

        let diff = end - start;
        let remaining = count - start;
        let phase = remaining % diff;
        for _ in 0..phase {
            platform.cycle();
        }

        Some(platform.load_north())
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
