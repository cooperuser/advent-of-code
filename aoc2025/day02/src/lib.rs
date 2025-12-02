use std::{collections::HashSet, ops::RangeInclusive};

use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    ranges: Vec<RangeInclusive<i64>>,
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

        for range in raw[0].split(',') {
            let (start, end) = range.split_once('-').unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();
            ranges.push(start..=end);
        }

        Self { raw, ranges }
    }

    fn part_a(&self) -> Option<i64> {
        let mut invalid: Vec<i64> = Vec::new();

        for range in &self.ranges {
            for i in range.clone() {
                let s = format!("{i}");
                if s.len() % 2 != 0 {
                    continue;
                }
                let (a, b) = s.split_at(s.len() / 2);
                if a == b {
                    invalid.push(i);
                }
            }
        }

        Some(invalid.iter().sum())
    }

    fn part_b(&self) -> Option<i64> {
        let mut invalid: Vec<i64> = Vec::new();

        for range in &self.ranges {
            let mut set: HashSet<i64> = HashSet::new();
            for i in range.clone() {
                let s = format!("{i}");
                let chars: Vec<_> = s.chars().collect();
                let len = s.len();

                for w in 2..=len {
                    if len % w != 0 {
                        continue;
                    }

                    let size = len / w;

                    let sub: HashSet<_> = chars.chunks(size).collect();
                    if sub.len() == 1 {
                        set.insert(i);
                    }
                }
            }

            for i in set {
                invalid.push(i);
            }
        }

        Some(invalid.iter().sum())
    }
}

utils::solution::test_solution!(aoc2025, day02);
