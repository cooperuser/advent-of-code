use std::collections::VecDeque;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 3749;
pub const ANSWER_B: i64 = 11387;

pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    equations: Vec<(i64, Vec<i64>)>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut equations = Vec::new();
        for line in raw.iter() {
            let (left, right) = line.split_once(':').unwrap();
            let left = left.parse().unwrap();
            let right = right
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            equations.push((left, right));
        }
        Self {
            raw: raw.clone(),
            equations,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut sum = 0;
        'equation: for equation in self.equations.iter() {
            let len = equation.1.len();
            let mut deque: VecDeque<(i64, usize)> = VecDeque::new();
            deque.push_back((equation.1[0], 1));
            while let Some(next) = deque.pop_front() {
                if next.1 == len && next.0 == equation.0 {
                    sum += equation.0;
                    continue 'equation;
                } else if next.1 == len {
                    continue;
                }

                deque.push_back((next.0 + equation.1[next.1], next.1 + 1));
                deque.push_back((next.0 * equation.1[next.1], next.1 + 1));
            }
        }
        Some(sum)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut sum = 0;
        'equation: for equation in self.equations.iter() {
            let len = equation.1.len();
            let mut deque: VecDeque<(i64, usize)> = VecDeque::new();
            deque.push_back((equation.1[0], 1));
            while let Some(next) = deque.pop_front() {
                if next.1 == len && next.0 == equation.0 {
                    sum += equation.0;
                    continue 'equation;
                } else if next.1 == len {
                    continue;
                }

                deque.push_back((next.0 + equation.1[next.1], next.1 + 1));
                deque.push_back((next.0 * equation.1[next.1], next.1 + 1));
                let right = equation.1[next.1];
                let magnitude = (right as f64).log10().ceil() as u32;
                deque.push_back((next.0 * 10i64.pow(magnitude) + right, next.1 + 1));
            }
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
