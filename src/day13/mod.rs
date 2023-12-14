#![allow(dead_code)]

use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 405;
pub const ANSWER_B: i64 = 400;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    patterns: Vec<Pattern>,
}

#[derive(Debug)]
struct Pattern {
    size: (i64, i64),
    rows: Vec<i64>,
    cols: Vec<i64>,
}

impl Pattern {
    fn score(&self, target: i64) -> i64 {
        let mut sum = 0;
        for row in 1..self.size.0 {
            if count_diffs(&self.rows, row as usize) == target {
                sum += row * 100;
            }
        }
        for col in 1..self.size.1 {
            if count_diffs(&self.cols, col as usize) == target {
                sum += col;
            }
        }
        sum
    }
}

fn count_row(points: &HashSet<(i64, i64)>, row: i64, max: i64) -> i64 {
    let mut value = 0;

    for col in 0..max {
        if points.contains(&(row, col)) {
            value |= 1 << col;
        }
    }

    value
}

fn count_col(points: &HashSet<(i64, i64)>, col: i64, max: i64) -> i64 {
    let mut value = 0;

    for row in 0..max {
        if points.contains(&(row, col)) {
            value |= 1 << row;
        }
    }

    value
}

fn count_diffs(nums: &Vec<i64>, slice: usize) -> i64 {
    let mut diffs = 0;
    for offset in 0..nums.len() {
        let left = (slice - offset - 1) as i64;
        let right = slice + offset;

        if left < 0 || right >= nums.len() {
            break;
        }

        let a = nums[left as usize];
        let b = nums[right];
        let c = a ^ b;

        let mut bit = 1;
        while bit <= c {
            if c & bit != 0 {
                diffs += 1;
            }
            bit <<= 1;
        }
    }
    diffs
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut patterns = Vec::new();

        for group in raw.split(|line| line.is_empty()) {
            let mut points = HashSet::new();
            for (row, line) in group.iter().enumerate() {
                for (col, ch) in line.chars().enumerate() {
                    if ch == '#' {
                        points.insert((row as i64, col as i64));
                    }
                }
            }

            let size = (group.len() as i64, group[0].len() as i64);
            patterns.push(Pattern {
                rows: (0..size.0).map(|row| count_row(&points, row, size.1)).collect(),
                cols: (0..size.1).map(|col| count_col(&points, col, size.0)).collect(),
                size,
            });
        }

        Self {
            raw: raw.clone(),
            patterns,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        Some(self.patterns.iter().map(|p| p.score(0)).sum())
    }

    pub fn part_b(&self) -> Option<i64> {
        Some(self.patterns.iter().map(|p| p.score(1)).sum())
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
