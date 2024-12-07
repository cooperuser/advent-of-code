#![allow(dead_code)]

use std::collections::HashSet;

use crate::direction::Direction;
use crate::vector::{Vector, VectorMap};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 41;
pub const ANSWER_B: i64 = 6;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    grid: Vec<Vec<bool>>,
    size: Vector,
    start: Vector,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut start = Vector::default();
        let mut grid = vec![vec![false; raw[0].len()]; raw.len()];
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let spot = (x, y).into();
                if c == '#' {
                    grid[y][x] = true;
                } else if c == '^' {
                    start = spot;
                }
            }
        }

        Self {
            raw: raw.clone(),
            grid,
            start,
            size: Vector::new_usize(raw[0].len(), raw.len()),
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        Some(self.compute_path().unwrap().len() as i64)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut total = 0;
        for spot in self.compute_path().unwrap() {
            if spot == self.start {
                continue;
            } else if self.does_path_loop(spot) {
                total += 1;
            }
        }

        Some(total)
    }

    fn compute_path(&self) -> Option<HashSet<Vector>> {
        let mut seen: HashSet<Vector> = HashSet::new();
        let mut facing = Direction::North;
        let mut pos = self.start;
        loop {
            seen.insert(pos);
            let next = pos + facing;
            if next.x < 0 || next.y < 0 || next.x >= self.size.x || next.y >= self.size.y {
                return Some(seen);
            } else if self.grid[next.y as usize][next.x as usize] {
                facing = facing.rotate_right();
            } else {
                pos = next;
            }
        }
    }

    fn does_path_loop(&self, extra: Vector) -> bool {
        let mut seen: VectorMap<Vec<bool>> = VectorMap::new(self.size + Vector::new(1, 1));
        let mut facing = Direction::North;
        let mut pos = self.start;
        loop {
            match seen.get(pos) {
                Some(dirs) => {
                    if dirs[facing as usize] {
                        return true;
                    }
                }
                None => {
                    seen.insert(pos, vec![false; 4]);
                }
            }

            seen.get_mut(pos).as_mut().unwrap()[facing as usize] = true;
            let next = pos + facing;
            if next.x < 0 || next.y < 0 || next.x >= self.size.x || next.y >= self.size.y {
                return false;
            } else if self.grid[next.y as usize][next.x as usize] || next == extra {
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
