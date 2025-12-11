use std::collections::{HashMap, HashSet, VecDeque};

use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    devices: HashMap<Rc<str>, HashSet<Rc<str>>>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample_b.txt").to_string(),
            answer_a: 5,
            answer_b: 2,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut devices = HashMap::new();
        for line in &raw {
            let (name, connections) = line.split_once(": ").unwrap();
            devices.insert(
                name.into(),
                connections.split_whitespace().map(|s| s.into()).collect(),
            );
        }
        Self { raw, devices }
    }

    fn part_a(&self) -> Option<i64> {
        let mut queue: VecDeque<(Rc<str>, HashSet<Rc<str>>)> =
            VecDeque::from([("you".into(), HashSet::new())]);
        let target: Rc<str> = "out".into();
        let mut paths = 0;
        while let Some((pos, seen)) = queue.pop_front() {
            if pos == target {
                paths += 1;
                continue;
            }

            let Some(connections) = self.devices.get(&pos) else {
                panic!("Not found: {pos}");
            };
            for c in connections {
                if seen.contains(c) {
                    continue;
                }
                let mut seen = seen.clone();
                seen.insert(c.clone());
                queue.push_back((c.clone(), seen));
            }
        }
        Some(paths)
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

utils::solution::test_solution!(aoc2025, day11);
