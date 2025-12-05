#![feature(slice_split_once)]

use std::ops::{Range, RangeInclusive};

use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    fresh: Vec<RangeInclusive<i64>>,
    available: Vec<i64>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 3,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let (raw_fresh, raw_available) = raw.split_once(|line| line.is_empty()).unwrap();
        let mut fresh = Vec::new();
        let mut available = Vec::new();

        for line in raw_fresh {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse().unwrap();
            let end = end.parse().unwrap();
            fresh.push(start..=end);
        }

        for line in raw_available {
            available.push(line.parse().unwrap());
        }

        Self {
            raw,
            fresh,
            available,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut count = 0;
        'outer: for a in &self.available {
            for r in &self.fresh {
                if r.contains(a) {
                    count += 1;
                    continue 'outer;
                }
            }
        }
        Some(count)
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

utils::solution::test_solution!(aoc2025, day05);
