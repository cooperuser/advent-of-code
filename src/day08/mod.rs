// #![allow(dead_code)]

use std::collections::{HashMap, HashSet};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample_a.txt");
pub const SAMPLE_B: &str = include_str!("input_sample_b.txt");
pub const ANSWER_A: i64 = 2;
pub const ANSWER_B: i64 = 6;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    dirs: Vec<bool>,
    map: HashMap<String, (String, String)>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut map = HashMap::new();
        let parts: Vec<&[String]> = raw.split(|line| line.is_empty()).collect();
        for line in parts[1] {
            let name = line[0..3].to_string();
            let left = line[7..10].to_string();
            let right = line[12..15].to_string();
            map.insert(name, (left, right));
        }
        Self {
            raw: raw.clone(),
            dirs: parts[0][0].chars().map(|c| c == 'R').collect(),
            map,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        Some(self.count("AAA", "ZZZ"))
    }

    pub fn part_b(&self) -> Option<i64> {
        let loops: Vec<u64> = self.map.keys()
            .filter(|p| p.ends_with('A'))
            .map(|p| self.count(p, "Z") as u64)
            .collect();
        let primes: HashSet<u64> = loops
            .into_iter()
            .flat_map(primes::factors_uniq)
            .collect();
        Some(primes.iter().product::<u64>() as i64)
    }

    fn count(&self, start: &str, end: &str) -> i64 {
        let mut place = start;
        let mut step = 0;
        while !place.ends_with(end) {
            let dir = self.dirs[step % self.dirs.len()];
            let next = self.map.get(place).unwrap();
            place = if dir { &next.1 } else { &next.0 };
            step += 1;
        }
        step as i64
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
