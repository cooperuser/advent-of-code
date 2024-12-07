#![allow(dead_code)]

use std::collections::VecDeque;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 3749;
pub const ANSWER_B: i64 = 11387;

pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    equations: Vec<Equation>,
}

struct Equation {
    target: i64,
    numbers: Vec<i64>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut equations = Vec::new();
        for line in raw.iter() {
            let (left, right) = line.split_once(':').unwrap();
            equations.push(Equation {
                target: left.parse().unwrap(),
                numbers: right
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect(),
            });
        }
        Self {
            raw: raw.clone(),
            equations,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        Some(self.equations.iter().map(|eq| self.helper(eq, false)).sum())
    }

    pub fn part_b(&self) -> Option<i64> {
        Some(self.equations.iter().map(|eq| self.helper(eq, true)).sum())
    }

    fn helper(&self, equation: &Equation, concatenate: bool) -> i64 {
        let mut deque: VecDeque<(i64, usize)> = VecDeque::new();
        deque.push_back((equation.numbers[0], 1));
        while let Some(partial) = deque.pop_front() {
            if partial.1 == equation.numbers.len() {
                if partial.0 == equation.target {
                    return equation.target;
                }
                continue;
            }

            deque.push_back((partial.0 + equation.numbers[partial.1], partial.1 + 1));
            deque.push_back((partial.0 * equation.numbers[partial.1], partial.1 + 1));
            if concatenate {
                let number = equation.numbers[partial.1];
                let magnitude = (number as f64).log10().ceil() as u32;
                deque.push_back((partial.0 * 10i64.pow(magnitude) + number, partial.1 + 1));
            }
        }

        0
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
