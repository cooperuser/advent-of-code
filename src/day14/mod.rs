#![allow(dead_code)]

use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 24;
pub const SAMPLE_B: i32 = 93;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    grid: HashSet<(i32, i32)>,
    bottom: i32
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut grid = HashSet::new();
        let mut bottom = 0;
        for line in &raw {
            if line.is_empty() { continue }
            let segments: Vec<&str> = line.split(" -> ").collect();
            let positions: Vec<(i32, i32)> = segments.iter().map(|seg| {
                let (x, y) = seg.split_once(",").unwrap();
                let (x, y): (i32, i32) = (x.parse().unwrap(), y.parse().unwrap());
                (x, y)
            }).collect();
            for window in positions.windows(2) {
                let a = window[0];
                let b = window[1];
                for x in a.0.min(b.0) ..= a.0.max(b.0) {
                    for y in a.1.min(b.1) ..= a.1.max(b.1) {
                        grid.insert((x, y));
                        bottom = bottom.max(y);
                    }
                }
            }
        }
        Self {
            raw: raw.clone(),
            grid,
            bottom,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut grid = self.grid.clone();
        let mut count = 0;
        let start = (500, 0);
        'outer: loop {
            let mut pos = start;
            loop {
                if pos.1 > self.bottom + 1 { break 'outer; }

                let down = (pos.0, pos.1 + 1);
                let left = (pos.0 - 1, pos.1 + 1);
                let right = (pos.0 + 1, pos.1 + 1);

                if !grid.contains(&down) {
                    pos = down;
                    continue;
                } else if !grid.contains(&left) {
                    pos = left;
                    continue;
                } else if !grid.contains(&right) {
                    pos = right;
                    continue;
                }

                grid.insert(pos);
                count += 1;
                break;
            }
        }
        count
    }

    pub fn part_b(&self) -> i32 {
        let mut grid = self.grid.clone();
        let mut count = 0;
        let start = (500, 0);
        while !grid.contains(&start) {
            let mut pos = start;
            loop {
                let down = (pos.0, pos.1 + 1);
                let left = (pos.0 - 1, pos.1 + 1);
                let right = (pos.0 + 1, pos.1 + 1);

                if pos.1 != self.bottom + 1 {
                    if !grid.contains(&down) {
                        pos = down;
                        continue;
                    } else if !grid.contains(&left) {
                        pos = left;
                        continue;
                    } else if !grid.contains(&right) {
                        pos = right;
                        continue;
                    }
                }

                grid.insert(pos);
                count += 1;
                break;
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
