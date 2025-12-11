use std::collections::{HashMap, HashSet};

use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    devices: HashMap<Rc<str>, HashSet<Rc<str>>>,
}

impl Solution<usize, usize> for Day {
    fn meta() -> Meta<usize, usize> {
        Meta::<usize, usize> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample_b.txt").to_string(),
            answer_a: 5,
            answer_b: 2,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let devices = raw
            .iter()
            .map(|line| {
                let (name, connections) = line.split_once(": ").unwrap();
                (
                    name.into(),
                    connections.split_whitespace().map(|s| s.into()).collect(),
                )
            })
            .collect();

        Self { raw, devices }
    }

    fn part_a(&self) -> Option<usize> {
        Some(self.count_paths(&mut HashMap::new(), "you".into(), true, true))
    }

    fn part_b(&self) -> Option<usize> {
        Some(self.count_paths(&mut HashMap::new(), "svr".into(), false, false))
    }
}

impl Day {
    fn count_paths(
        &self,
        cache: &mut HashMap<(Rc<str>, bool, bool), usize>,
        pos: Rc<str>,
        dac: bool,
        fft: bool,
    ) -> usize {
        if let Some(&count) = cache.get(&(pos.clone(), dac, fft)) {
            return count;
        }

        let (dac, fft) = match pos.to_string().as_str() {
            "out" => return (dac && fft) as usize,
            "dac" => (true, fft),
            "fft" => (dac, true),
            _ => (dac, fft),
        };

        let count = self
            .devices
            .get(&pos)
            .unwrap()
            .iter()
            .map(|next| self.count_paths(cache, next.clone(), dac, fft))
            .sum();
        cache.insert((pos, dac, fft), count);
        count
    }
}

utils::solution::test_solution!(aoc2025, day11);
