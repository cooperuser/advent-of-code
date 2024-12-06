use std::collections::HashSet;

use crate::direction::Direction;
use crate::vector::Vector;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 41;
pub const ANSWER_B: i64 = 6;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    map: HashSet<Vector>,
    size: Vector,
    start: Vector,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut map = HashSet::new();
        let mut start = Vector::default();
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let spot = (x, y).into();
                if c == '#' {
                    map.insert(spot);
                } else if c == '^' {
                    start = spot;
                }
            }
        }

        Self {
            raw: raw.clone(),
            map,
            start,
            size: Vector::new_usize(raw[0].len(), raw.len()),
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        Some(self.compute_path(None).unwrap().len() as i64)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut total = 0;
        for spot in self.compute_path(None).unwrap() {
            if spot == self.start {
                continue;
            } else if self.compute_path(Some(spot)).is_none() {
                total += 1;
            }
        }

        Some(total)
    }

    fn compute_path(&self, extra: Option<Vector>) -> Option<HashSet<Vector>> {
        let mut facing = Direction::North;
        let mut seen: HashSet<Vector> = HashSet::new();
        let mut seen_facing: HashSet<(Vector, Direction)> = HashSet::new();
        let mut pos = self.start;
        loop {
            if seen_facing.contains(&(pos, facing)) {
                return None;
            }
            seen.insert(pos);
            seen_facing.insert((pos, facing));
            let next = pos + facing;
            if next.x < 0 || next.y < 0 || next.x >= self.size.x || next.y >= self.size.y {
                return Some(seen);
            } else if self.map.contains(&next) || Some(next) == extra {
                facing = facing.rotate_right();
            } else {
                pos = next;
            }
        }
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
