#![allow(dead_code)]

use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 18;
pub const ANSWER_B: i64 = 9;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    grid: Vec<Vec<char>>,
    length: i64,
    width: i64,
}

type Point = (i64, i64);
const DIRS: &[Point] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            grid: raw.iter().map(|line| line.chars().collect()).collect(),
            length: raw.len() as i64,
            width: raw[0].len() as i64,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let word = &['X', 'M', 'A', 'S'];
        let mut total = 0;

        for y in 0..self.length {
            for x in 0..self.width {
                'dir: for dir in DIRS {
                    let mut p = (y, x);
                    for letter in word {
                        if p.0 < 0
                            || p.1 < 0
                            || p.0 >= self.length
                            || p.1 >= self.width
                            || self.grid[p.0 as usize][p.1 as usize] != *letter
                        {
                            continue 'dir;
                        }
                        p = (p.0 + dir.0, p.1 + dir.1);
                    }

                    total += 1;
                }
            }
        }

        Some(total)
    }

    pub fn part_b(&self) -> Option<i64> {
        let word = &['M', 'A', 'S'];
        let mut total = 0;
        let mut spots: HashSet<Point> = HashSet::new();

        for y in 0..self.length {
            for x in 0..self.width {
                'dir: for dir in DIRS {
                    if dir.0 == 0 || dir.1 == 0 {
                        continue;
                    }

                    let mut p = (y, x);
                    for letter in word {
                        if p.0 < 0
                            || p.1 < 0
                            || p.0 >= self.length
                            || p.1 >= self.width
                            || self.grid[p.0 as usize][p.1 as usize] != *letter
                        {
                            continue 'dir;
                        }
                        p = (p.0 + dir.0, p.1 + dir.1);
                    }

                    let spot = (y + dir.0, x + dir.1);
                    if !spots.insert(spot) {
                        total += 1;
                    }
                }
            }
        }

        Some(total)
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
