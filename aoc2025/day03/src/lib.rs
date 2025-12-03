use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    banks: Vec<Vec<i64>>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 357,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut banks = Vec::new();

        for line in &raw {
            let mut bank: Vec<i64> = Vec::new();

            for c in line.chars() {
                bank.push(c.to_string().parse().unwrap());
            }

            banks.push(bank);
        }

        Self { raw, banks }
    }

    fn part_a(&self) -> Option<i64> {
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

    fn part_b(&self) -> Option<i64> {
        None
    }
}

utils::solution::test_solution!(aoc2025, day03);
