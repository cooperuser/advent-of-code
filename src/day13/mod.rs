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
    rows: Vec<i64>,
    cols: Vec<i64>,
    size: (usize, usize),
}

impl Pattern {
    fn score(&self, target: i64) -> i64 {
        let mut sum = 0;
        sum += (1..self.size.0)
            .filter(|row| count_diffs(&self.rows, *row) == target)
            .map(|row| row * 100)
            .sum::<usize>();
        sum += (1..self.size.1)
            .filter(|col| count_diffs(&self.cols, *col) == target)
            .sum::<usize>();
        sum as i64
    }
}

fn count_row(points: &HashSet<(usize, usize)>, row: usize, max: usize) -> i64 {
    (0..max)
        .filter(|col| points.contains(&(row, *col)))
        .map(|col| 1 << col)
        .sum()
}

fn count_col(points: &HashSet<(usize, usize)>, col: usize, max: usize) -> i64 {
    (0..max)
        .filter(|row| points.contains(&(*row, col)))
        .map(|row| 1 << row)
        .sum()
}

fn count_diffs(nums: &Vec<i64>, slice: usize) -> i64 {
    let mut diffs = 0;
    for offset in 0..nums.len() {
        let left = slice - offset - 1;
        let right = slice + offset;

        if slice < offset + 1 || right >= nums.len() {
            break;
        }

        let a = nums[left];
        let b = nums[right];
        let c = a ^ b;

        let mut bit = 1;
        while bit <= c {
            if c & bit != 0 {
                diffs += 1;
            }

            // We don't care if we have more than 1 diff
            if diffs > 1 {
                return diffs;
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
                        points.insert((row, col));
                    }
                }
            }

            let size = (group.len(), group[0].len());
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
