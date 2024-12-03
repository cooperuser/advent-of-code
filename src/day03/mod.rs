#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample_a.txt");
pub const SAMPLE_B: &str = include_str!("input_sample_b.txt");
pub const ANSWER_A: i64 = 161;
pub const ANSWER_B: i64 = 48;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    input: String,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            input: raw.join("\n"),
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut sum = 0;

        for mul in self.input.split("mul(") {
            let Some(params) = mul.split_once(")") else {
                continue;
            };
            let Some((a, b)) = params.0.split_once(",") else {
                continue;
            };
            if !(1..=3).contains(&a.len()) || !(1..=3).contains(&b.len()) {
                continue;
            }
            let Some(a): Option<i64> = a.parse().ok() else {
                continue;
            };
            let Some(b): Option<i64> = b.parse().ok() else {
                continue;
            };
            sum += a * b;
        }

        Some(sum)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut sum = 0;

        let blocks = self
            .input
            .split("do()")
            .map(|d| match d.split_once("don't()") {
                Some(do_dont) => do_dont.0,
                _ => d,
            });

        for block in blocks {
            for mul in block.split("mul(") {
                let Some(params) = mul.split_once(")") else {
                    continue;
                };
                let Some((a, b)) = params.0.split_once(",") else {
                    continue;
                };
                if !(1..=3).contains(&a.len()) || !(1..=3).contains(&b.len()) {
                    continue;
                }
                let Some(a): Option<i64> = a.parse().ok() else {
                    continue;
                };
                let Some(b): Option<i64> = b.parse().ok() else {
                    continue;
                };
                sum += a * b;
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
