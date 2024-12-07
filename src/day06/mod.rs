#![allow(dead_code)]

use crate::direction::Direction;
use crate::vector::{Vector, VectorMap, VectorSet};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 41;
pub const ANSWER_B: i64 = 6;

pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    grid: VectorSet,
    size: Vector,
    start: Vector,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut grid = VectorSet::new(size);
        let mut start: Option<Vector> = None;
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let spot = Vector::new_usize(x, y);
                if c == '#' {
                    grid.insert(spot);
                } else if c == '^' {
                    start = Some(spot);
                }
            }
        }

        Self {
            raw: raw.clone(),
            grid,
            size,
            start: start.unwrap(),
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        Some(self.compute_path(None).unwrap().len() as i64)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut total = 0;
        for (spot, _) in self.compute_path(None).unwrap() {
            if spot == self.start {
                continue;
            } else if self.compute_path(Some(spot)).is_none() {
                total += 1;
            }
        }

        Some(total)
    }

    fn compute_path(&self, extra: Option<Vector>) -> Option<VectorMap<Vec<bool>>> {
        let mut seen: VectorMap<Vec<bool>> = VectorMap::new(self.size);
        let mut facing = Direction::North;
        let mut pos = self.start;
        loop {
            match seen.get(pos) {
                Some(dirs) => {
                    if dirs[facing as usize] {
                        return None;
                    }
                }
                None => {
                    seen.insert(pos, vec![false; 4]);
                }
            }

            seen.get_mut(pos).unwrap()[facing as usize] = true;
            let next = pos + facing;
            if !next.contained_in(Vector::zero(), self.size) {
                return Some(seen);
            } else if self.grid.contains(next) || Some(next) == extra {
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
