#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 374;
pub const ANSWER_B: i64 = 82000210;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    map: HashMap<usize, ((i64, i64), (i64, i64))>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut set: HashSet<(i64, i64)> = HashSet::new();
        let mut rows: HashSet<usize> = (0..raw.len()).collect();
        let mut cols: HashSet<usize> = (0..raw[0].len()).collect();
        let mut map = HashMap::new();

        for (row, line) in raw.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#' {
                    rows.remove(&row);
                    cols.remove(&col);
                    set.insert((row as i64, col as i64));
                }
            }
        }

        for (index, galaxy) in set.iter().enumerate() {
            let rows = rows.iter().filter(|r| **r < galaxy.0 as usize).count();
            let cols = cols.iter().filter(|c| **c < galaxy.1 as usize).count();
            map.insert(index, (*galaxy, (rows as i64, cols as i64)));
        }

        Self {
            raw: raw.clone(),
            map
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        Some(self.sum_dists(2))
    }

    pub fn part_b(&self) -> Option<i64> {
        Some(self.sum_dists(1_000_000))
    }

    fn sum_dists(&self, scale: i64) -> i64 {
        let scale = scale - 1;
        let mut sum = 0;

        for x in 0..self.map.len() - 1 {
            for y in x + 1..self.map.len() {
                let a = self.map.get(&x).unwrap();
                let b = self.map.get(&y).unwrap();

                let a = (a.0.0 + a.1.0 * scale, a.0.1 + a.1.1 * scale);
                let b = (b.0.0 + b.1.0 * scale, b.0.1 + b.1.1 * scale);

                sum += (a.0 - b.0).abs() + (a.1 - b.1).abs();
            }
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE_A));
        assert_eq!(solution.part_a().unwrap_or(0), ANSWER_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE_B));
        assert_eq!(solution.part_b().unwrap_or(0), ANSWER_B);
    }
}
