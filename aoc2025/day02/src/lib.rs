use std::ops::RangeInclusive;

use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    ranges: Vec<RangeInclusive<i64>>,
    max: i64,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 1227775554,
            answer_b: 4174379265,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut ranges = Vec::new();
        let mut max = 0;

        for range in raw[0].split(',') {
            let (start, end) = range.split_once('-').unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();
            max = max.max(end);
            ranges.push(start..=end);
        }

        Self { raw, ranges, max }
    }

    fn part_a(&self) -> Option<i64> {
        let patterns = Self::gen_patterns_a(self.max.ilog10());
        let mut invalid = 0;

        for range in &self.ranges {
            'step: for step in range.clone() {
                let len = step.ilog10() as usize;
                for pattern in patterns.get(len).unwrap() {
                    if step % pattern == 0 {
                        invalid += step;
                        continue 'step;
                    }
                }
            }
        }

        Some(invalid)
    }

    fn part_b(&self) -> Option<i64> {
        let patterns = Self::gen_patterns_b(self.max.ilog10());
        let mut invalid = 0;

        for range in &self.ranges {
            'step: for step in range.clone() {
                let len = step.ilog10() as usize;
                for pattern in patterns.get(len).unwrap() {
                    if step % pattern == 0 {
                        invalid += step;
                        continue 'step;
                    }
                }
            }
        }

        Some(invalid)
    }
}

impl Day {
    fn gen_patterns_a(max: u32) -> Vec<Vec<i64>> {
        let mut all_patterns: Vec<Vec<i64>> = Vec::new();

        for len in 0..=max + 1 {
            let mut patterns = Vec::new();

            if len % 2 != 0 {
                patterns.push(1 + 10i64.pow(len / 2 + 1));
            }

            all_patterns.push(patterns);
        }

        all_patterns
    }

    fn gen_patterns_b(max: u32) -> Vec<Vec<i64>> {
        let mut all_patterns: Vec<Vec<i64>> = Vec::new();

        for len in 1..=max + 1 {
            let mut patterns = Vec::new();

            for divisor in 1..=len / 2 {
                if len % divisor == 0 {
                    let mut pattern = 1;
                    for i in 1..len / divisor {
                        pattern += 10i64.pow(i * divisor);
                    }
                    patterns.push(pattern);
                }
            }

            all_patterns.push(patterns);
        }

        all_patterns
    }
}

utils::solution::test_solution!(aoc2025, day02);
