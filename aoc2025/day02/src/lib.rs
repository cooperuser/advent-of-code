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
        let mut invalid = 0;

        for range in &self.ranges {
            for step in range.clone() {
                let string = format!("{step}");
                if string.len() % 2 != 0 {
                    continue;
                }
                let (a, b) = string.split_at(string.len() / 2);
                if a == b {
                    invalid += step;
                }
            }
        }

        Some(invalid)
    }

    fn part_b(&self) -> Option<i64> {
        let mut invalid = 0;

        for range in &self.ranges {
            let mut set: HashSet<i64> = HashSet::new();
            for step in range.clone() {
                let string = format!("{step}");
                let chars: Vec<_> = string.chars().collect();
                let len = string.len();

                'parts: for parts in 2..=len {
                    if len % parts != 0 {
                        continue;
                    }

                    let mut chunks = chars.chunks(len / parts);
                    let first = chunks.next().unwrap();
                    for chunk in chunks {
                        if chunk != first {
                            continue 'parts;
                        }
                    }

                    set.insert(step);
                }
            }

            for i in set {
                invalid += i;
            }
        }

        Some(invalid)
    }
}

utils::solution::test_solution!(aoc2025, day02);
