use std::collections::VecDeque;

use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    machines: Vec<Machine>,
}

#[derive(Debug)]
struct Machine {
    indicators: Vec<bool>,
    schematics: Vec<Vec<usize>>,
    requirements: Vec<i64>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 7,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut machines = Vec::new();

        for line in &raw {
            let parts: Vec<_> = line.split_whitespace().collect();
            let raw_indicators: Vec<_> = parts[0].chars().collect();
            let raw_requirements = parts[parts.len() - 1];
            let raw_requirements = &raw_requirements[1..raw_requirements.len() - 1];
            let raw_schematics: Vec<_> = parts.iter().skip(1).take(parts.len() - 2).collect();
            let mut indicators = Vec::new();
            for &indicator in &raw_indicators[1..raw_indicators.len() - 1] {
                indicators.push(indicator == '#');
            }
            let mut requirements = Vec::new();
            for requirement in raw_requirements.split(',') {
                requirements.push(requirement.parse().unwrap());
            }
            let mut schematics: Vec<Vec<_>> = Vec::new();
            for s in raw_schematics {
                let s = &s[1..s.len() - 1];
                schematics.push(s.split(',').map(|n| n.parse().unwrap()).collect());
            }

            machines.push(Machine {
                indicators,
                schematics,
                requirements,
            })
        }

        Self { raw, machines }
    }

    fn part_a(&self) -> Option<i64> {
        let mut presses = 0;
        for machine in &self.machines {
            let mut queue = VecDeque::new();
            queue.push_back((0, vec![false; machine.indicators.len()]));
            while let Some(state) = queue.pop_front() {
                if state.1 == machine.indicators {
                    presses += state.0;
                    break;
                }

                for button in &machine.schematics {
                    let mut indicators = state.1.clone();
                    for &b in button {
                        indicators[b] ^= true;
                    }
                    queue.push_back((state.0 + 1, indicators));
                }
            }
        }
        Some(presses)
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

utils::solution::test_solution!(aoc2025, day10);
