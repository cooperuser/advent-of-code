#![allow(dead_code)]

use std::collections::{BTreeSet, HashMap, HashSet};

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
    rocks: HashSet<(i64, i64)>,
    map: Vec<Vec<Option<Rock>>>,
    size: (i64, i64),
}

#[derive(Clone)]
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

        let mut rocks = HashSet::new();
        let mut map: Vec<Vec<Option<Rock>>> = (0..self.size.0)
            .map(|_| (0..self.size.1).map(|_| None).collect())
            .collect();
        for &row in &rows {
            for &col in &cols {
                match self.map[row as usize][col as usize] {
                    Some(Rock::Square) => map[row as usize][col as usize] = Some(Rock::Square),
                    Some(Rock::Round) => {
                        let mut r = row;
                        let mut c = col;
                        while r + dr >= 0
                            && c + dc >= 0
                            && r + dr < self.size.0
                            && c + dc < self.size.1
                        {
                            if map[(r + dr) as usize][(c + dc) as usize].is_some() {
                                break;
                            }
                            r += dr;
                            c += dc;
                        }
                        map[r as usize][c as usize] = Some(Rock::Round);
                        rocks.insert((r, c));
                    }
                    None => (),
                }
            }
        }
        self.rocks = rocks;
        self.map = map;
    }

    fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    fn serialize(&self) -> BTreeSet<(i64, i64)> {
        self.rocks.iter().cloned().collect()
    }

    fn load_north(&self) -> i64 {
        self.rocks.iter().map(|(row, _)| self.size.0 - row).sum()
    }
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut rocks = HashSet::new();
        let mut map = Vec::new();
        for (row, line) in raw.iter().enumerate() {
            let mut map_row = Vec::new();
            for (col, ch) in line.chars().enumerate() {
                let pos = (row as i64, col as i64);
                match ch {
                    '#' => {
                        rocks.insert(pos);
                        map_row.push(Some(Rock::Square));
                    }
                    'O' => {
                        rocks.insert(pos);
                        map_row.push(Some(Rock::Round));
                    }
                    _ => {
                        map_row.push(None);
                    }
                }
            }
            map.push(map_row);
        }
        Self {
            raw: raw.clone(),
            platform: Platform {
                rocks,
                map,
                size: (raw.len() as i64, raw[0].len() as i64),
            },
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
