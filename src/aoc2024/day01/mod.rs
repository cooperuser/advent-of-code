use crate::prelude::*;

#[derive(Default)]
pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    left: Vec<i64>,
    right: Vec<i64>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 11,
            answer_b: 31,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
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

    fn part_a(&self) -> Option<i64> {
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

    fn part_b(&self) -> Option<i64> {
        let mut sum = 0;
        for value in &self.left {
            let count = self.right.iter().filter(|n| *n == value).count();
            sum += value * count as i64;
        }
        Some(sum)
    }
}

crate::solution::test_solution!();
