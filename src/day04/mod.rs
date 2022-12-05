#![allow(dead_code)]

use std::{ops::Range, collections::HashSet};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 2;
pub const SAMPLE_B: i32 = 4;

#[derive(Default)]
pub struct Solution {
    raw: Vec<String>,
    lines: Vec<(Range<i32>, Range<i32>)>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            lines: raw.iter().map(|line| {
                let ranges = line.split(",")
                    .map(|side| {
                        let v = side.split("-")
                            .map(|num| num.parse::<i32>().unwrap())
                            .collect::<Vec<i32>>();
                        v[0]..v[1] + 1
                    }).collect::<Vec<Range<i32>>>();
                (ranges[0].clone(), ranges[1].clone())
            }).collect(),
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut count = 0;
        for line in self.lines.clone() {
            let a = line.0.collect::<HashSet<i32>>();
            let b = line.1.collect::<HashSet<i32>>();
            let inter = a.intersection(&b).collect::<HashSet<&i32>>().len();
            if inter == a.len().min(b.len()) {
                count += 1;
            }
        }
        count
    }

    pub fn part_b(&self) -> i32 {
        let mut count = 0;
        for line in self.lines.clone() {
            let a = line.0.collect::<HashSet<i32>>();
            let b = line.1.collect::<HashSet<i32>>();
            let union = a.union(&b).collect::<HashSet<&i32>>().len();
            if union != a.len() + b.len() {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_a(), SAMPLE_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_b(), SAMPLE_B);
    }
}
