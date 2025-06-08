use crate::prelude::*;

#[derive(Default)]
pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    input: Rc<str>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample_a.txt").to_string(),
            sample_b: include_str!("input_sample_b.txt").to_string(),
            answer_a: 161,
            answer_b: 48,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        Self {
            raw: raw.clone(),
            input: raw.join("\n").into(),
        }
    }

    fn part_a(&self) -> Option<i64> {
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

    fn part_b(&self) -> Option<i64> {
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

crate::solution::test_solution!();
