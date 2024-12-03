#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 11;
pub const ANSWER_B: i64 = 31;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    left: Vec<i64>,
    right: Vec<i64>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            left: raw
                .iter()
                .map(|s| s.split_whitespace().next().unwrap().parse().unwrap())
                .collect(),
            right: raw
                .iter()
                .map(|s| s.split_whitespace().nth(1).unwrap().parse().unwrap())
                .collect(),
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut sum = 0;
        let mut left = self.left.clone();
        let mut right = self.right.clone();
        left.sort();
        right.sort();
        for i in 0..self.raw.len() {
            sum += (left[i] - right[i]).abs();
        }
        Some(sum)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut sum = 0;
        for value in &self.left {
            let count = self.right.iter().filter(|n| *n == value).count();
            sum += value * count as i64;
        }
        Some(sum)
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
