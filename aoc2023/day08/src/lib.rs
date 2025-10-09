use std::collections::{HashMap, HashSet};
use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    dirs: Vec<bool>,
    map: HashMap<Rc<str>, (Rc<str>, Rc<str>)>,
}

impl Solution<usize, usize> for Day {
    fn meta() -> Meta<usize, usize> {
        Meta::<usize, usize> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample_a.txt").to_string(),
            sample_b: include_str!("input_sample_b.txt").to_string(),
            answer_a: 6,
            answer_b: 6,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut map = HashMap::new();
        for line in raw.iter().skip(2) {
            let name = line[0..3].into();
            let left = line[7..10].into();
            let right = line[12..15].into();
            map.insert(name, (left, right));
        }

        Self {
            raw: raw.clone(),
            dirs: raw[0].chars().map(|c| c == 'R').collect(),
            map,
        }
    }

    fn part_a(&self) -> Option<usize> {
        Some(self.count("AAA", "ZZZ"))
    }

    fn part_b(&self) -> Option<usize> {
        let loops: Vec<u64> = self
            .map
            .keys()
            .filter(|p| p.ends_with('A'))
            .map(|p| self.count(p, "Z") as u64)
            .collect();
        let primes: HashSet<_> = loops.into_iter().flat_map(primes::factors_uniq).collect();
        Some(primes.iter().product::<u64>() as usize)
    }
}

impl Day {
    fn count(&self, start: &str, end: &str) -> usize {
        let mut place = start;
        let mut step = 0;

        while !place.ends_with(end) {
            let dir = self.dirs[step % self.dirs.len()];
            let next = self.map.get(place).unwrap();
            place = match dir {
                true => &next.1,
                false => &next.0,
            };
            step += 1;
        }

        step
    }
}

utils::solution::test_solution!(aoc2023, day08);
