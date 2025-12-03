use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    banks: Vec<Vec<u64>>,
}

impl Solution<u64, u64> for Day {
    fn meta() -> Meta<u64, u64> {
        Meta::<u64, u64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 357,
            answer_b: 3121910778619,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let banks = raw
            .iter()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect()
            })
            .collect();

        Self { raw, banks }
    }

    fn part_a(&self) -> Option<u64> {
        let mut total = 0;

        for bank in &self.banks {
            let max = bank.iter().max().unwrap();
            if bank.iter().filter(|&n| n == max).count() > 1 {
                total += max * 10 + max;
                continue;
            }

            let pos = bank.iter().position(|n| n == max).unwrap();
            let left = bank.iter().take(pos);
            let right = bank.iter().skip(pos + 1);

            let joltage = match (left.max(), right.max()) {
                (Some(a), Some(b)) => (a * 10 + max).max(max * 10 + b),
                (Some(a), None) => a * 10 + max,
                (None, Some(b)) => max * 10 + b,
                _ => unreachable!(),
            };

            total += joltage;
        }

        Some(total)
    }

    fn part_b(&self) -> Option<u64> {
        let mut total = 0;

        for bank in &self.banks {
            let mut joltage = 0;
            let mut last = 0;

            for i in 0..12 {
                let max = bank
                    .iter()
                    .enumerate()
                    .skip(last)
                    .take(bank.len() - last - (11 - i))
                    .rev()
                    .max_by_key(|m| m.1)
                    .unwrap();

                last = max.0 + 1;
                joltage = joltage * 10 + max.1;
            }

            total += joltage;
        }

        Some(total)
    }
}

utils::solution::test_solution!(aoc2025, day03);
