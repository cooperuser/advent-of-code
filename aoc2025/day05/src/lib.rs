#![feature(slice_split_once)]

use std::ops::Range;

use utils::{prelude::*, ranges::merge};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    fresh: Vec<Range<i64>>,
    available: Vec<i64>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 3,
            answer_b: 14,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let (raw_fresh, raw_available) = raw.split_once(|line| line.is_empty()).unwrap();
        let mut fresh = Vec::new();
        let mut available = Vec::new();

        for line in raw_fresh {
            let (start, end) = line.split_once('-').unwrap();
            let start = start.parse().unwrap();
            let end = end.parse::<i64>().unwrap() + 1;
            fresh.push(Range { start, end });
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
        let mut ranges = merge(&self.fresh).into_iter();
        let mut available = self.available.clone();
        available.sort();

        let mut count = 0;
        let mut range = ranges.next().unwrap();

        for a in available {
            while a > range.end {
                range = match ranges.next() {
                    Some(r) => r,
                    None => break,
                }
            }

            if a >= range.start && a <= range.end {
                count += 1;
            }
        }

        Some(count)
    }

    fn part_b(&self) -> Option<i64> {
        let ranges = merge(&self.fresh);
        Some(ranges.iter().map(|r| r.end - r.start).sum())
    }
}

utils::solution::test_solution!(aoc2025, day05);
