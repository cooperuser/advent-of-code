#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 46;
pub const ANSWER_B: i64 = 51;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    mirrors: HashMap<Pos, Mirror>,
    size: (i64, i64),
}

type Pos = (i64, i64);

#[derive(Debug)]
enum Mirror {
    Vertical,
    Horizontal,
    Forward,
    Backward,
}

impl Mirror {
    fn reflect(&self, pos: Pos, dir: Dir) -> Vec<(Pos, Dir)> {
        let dirs = match (self, dir) {
            (Mirror::Vertical, Dir::North) => vec![Dir::North],
            (Mirror::Vertical, Dir::South) => vec![Dir::South],
            (Mirror::Vertical, Dir::East | Dir::West) => vec![Dir::North, Dir::South],
            (Mirror::Horizontal, Dir::North | Dir::South) => vec![Dir::East, Dir::West],
            (Mirror::Horizontal, Dir::East) => vec![Dir::East],
            (Mirror::Horizontal, Dir::West) => vec![Dir::West],
            (Mirror::Forward, Dir::North) => vec![Dir::East],
            (Mirror::Forward, Dir::South) => vec![Dir::West],
            (Mirror::Forward, Dir::East) => vec![Dir::North],
            (Mirror::Forward, Dir::West) => vec![Dir::South],
            (Mirror::Backward, Dir::North) => vec![Dir::West],
            (Mirror::Backward, Dir::South) => vec![Dir::East],
            (Mirror::Backward, Dir::East) => vec![Dir::South],
            (Mirror::Backward, Dir::West) => vec![Dir::North],
        };
        dirs.iter().map(|d| (d.add(pos), *d)).collect()
    }
}

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
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut map = HashMap::new();
        for (row, line) in raw.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if let Some(mirror) = match ch {
                    '|' => Some(Mirror::Vertical),
                    '-' => Some(Mirror::Horizontal),
                    '/' => Some(Mirror::Forward),
                    '\\' => Some(Mirror::Backward),
                    '.' => None,
                    _ => panic!("unknown ch {ch}"),
                } {
                    map.insert((row as i64, col as i64), mirror);
                }
            }
        }
        Self {
            raw: raw.clone(),
            mirrors: map,
            size: (raw.len() as i64, raw[0].len() as i64),
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        Some(self.get_energized((0, 0), Dir::East))
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut max = 0;
        for col in 0..self.size.1 {
            max = max.max(self.get_energized((0, col), Dir::South));
            max = max.max(self.get_energized((self.size.0 - 1, col), Dir::North));
        }
        for row in 0..self.size.0 {
            max = max.max(self.get_energized((row, 0), Dir::East));
            max = max.max(self.get_energized((row, self.size.1 - 1), Dir::West));
        }
        Some(max)
    }

    fn get_energized(&self, start_pos: Pos, start_dir: Dir) -> i64 {
        let mut lasers: HashSet<(Pos, Dir)> = HashSet::from([(start_pos, start_dir)]);
        let mut energized: HashSet<(Pos, Dir)> = HashSet::from([(start_pos, start_dir)]);
        let mut len = 0;
        while len != energized.len() {
            len = energized.len();
            let mut lasers_next = HashSet::new();
            for (pos, dir) in &lasers {
                if pos.0 < 0 || pos.0 >= self.size.0 || pos.1 < 0 || pos.1 >= self.size.1 {
                    continue;
                }
                energized.insert((*pos, *dir));
                if let Some(mirror) = self.mirrors.get(pos) {
                    for l in mirror.reflect(*pos, *dir) {
                        if energized.insert(l) {
                            lasers_next.insert(l);
                        }
                    }
                } else {
                    let next = dir.add(*pos);
                    if energized.insert((next, *dir)) {
                        lasers_next.insert((next, *dir));
                    }
                }
            }
            lasers = lasers_next;
        }

        energized
            .iter()
            .map(|(pos, _)| *pos)
            .filter(|pos| pos.0 >= 0 && pos.0 < self.size.0 && pos.1 >= 0 && pos.1 < self.size.1)
            .collect::<HashSet<_>>()
            .len() as i64
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
