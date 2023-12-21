#![allow(dead_code)]

use std::collections::{HashMap, VecDeque};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 11687500;
pub const ANSWER_B: i64 = 1;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    modules: HashMap<String, Module>,
    destinations: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone)]
enum Module {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcast,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut modules = HashMap::new();
        let mut destinations = HashMap::new();
        for line in &raw {
            if line.is_empty() {
                continue;
            }

            let (name, dests) = line.split_once(" -> ").unwrap();
            let dests: Vec<String> = dests.split(", ").map(|s| s.to_string()).collect();
            let (module, start) = match &name[0..1] {
                "%" => (Module::FlipFlop(false), 1),
                "&" => (Module::Conjunction(HashMap::new()), 1),
                _ => (Module::Broadcast, 0),
            };

            modules.insert(name[start..].to_string(), module);
            destinations.insert(name[start..].to_string(), dests);
        }

        for (label, dests) in &destinations {
            for dest in dests {
                if let Some(Module::Conjunction(memory)) = modules.get_mut(dest) {
                    memory.insert(label.clone(), false);
                }
            }
        }

        Self {
            raw: raw.clone(),
            modules,
            destinations,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut modules = self.modules.clone();
        let mut pulses = [0, 0];

        for _ in 0..1000 {
            let (p, _) = self.pulse(&mut modules, "");
            pulses[0] += p[0];
            pulses[1] += p[1];
        }

        Some(pulses[0] * pulses[1])
    }

    pub fn part_b(&self) -> Option<i64> {
        if self.modules.len() < 10 {
            return Some(1);
        }

        let inputs = if let Some(Module::Conjunction(memory)) = self.modules.get("cl") {
            memory.keys().collect::<Vec<_>>()
        } else {
            panic!("Could not find inputs to rx")
        };

        let mut total = 1;
        for input in &inputs {
            let mut modules = self.modules.clone();
            let mut presses = 0;
            loop {
                presses += 1;
                if self.pulse(&mut modules, input).1 {
                    total *= presses;
                    break;
                }
            }
        }

        Some(total)
    }

    fn pulse(&self, modules: &mut HashMap<String, Module>, target: &str) -> ([i64; 2], bool) {
        let mut pulses = [0, 0];
        let mut queue = VecDeque::from([("broadcaster", false, "button")]);
        while let Some((label, pulse, from)) = queue.pop_front() {
            pulses[pulse as usize] += 1;

            if let Some(dests) = self.destinations.get(label) {
                let output = match modules.get_mut(label).unwrap() {
                    Module::Broadcast => pulse,
                    Module::FlipFlop(state) => {
                        if pulse {
                            continue;
                        }
                        *state = !*state;
                        *state
                    }
                    Module::Conjunction(memory) => {
                        memory.insert(from.to_string(), pulse);
                        let any_low = memory.values().any(|p| !p);
                        if any_low && label == target {
                            return ([0, 0], true);
                        }
                        any_low
                    }
                };
                for dest in dests {
                    queue.push_back((dest, output, label));
                }
            }
        }
        (pulses, false)
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
