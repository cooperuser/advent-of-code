#![allow(dead_code)]
#![allow(clippy::ptr_arg)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 114;
pub const ANSWER_B: i64 = 2;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    seqs: Vec<Vec<i64>>,
}

fn get_next(nums: &Vec<i64>) -> i64 {
    let first = *nums.first().unwrap();
    let last = *nums.last().unwrap();
    if first == last { return first }
    last + get_next(&nums.windows(2).map(|w| w[1] - w[0]).collect())
}

fn get_prev(nums: &Vec<i64>) -> i64 {
    let first = *nums.first().unwrap();
    let last = *nums.last().unwrap();
    if first == last { return first }
    first - get_prev(&nums.windows(2).map(|w| w[1] - w[0]).collect())
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            seqs: raw.iter()
                .map(|line| line
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
                ).collect()
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        Some(self.seqs.iter().map(get_next).sum())
    }

    pub fn part_b(&self) -> Option<i64> {
        Some(self.seqs.iter().map(get_prev).sum())
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
