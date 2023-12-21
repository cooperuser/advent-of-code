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
        let mut pulses = [0, 0];

        let mut modules = self.modules.clone();
        for _ in 0..1000 {
            let mut queue = VecDeque::from([("broadcaster", false, "button")]);
            while let Some((label, pulse, from)) = queue.pop_front() {
                pulses[pulse as usize] += 1;

                if self.destinations.get(label).is_none() {
                    continue;
                }
                let dests = self.destinations.get(label).unwrap();
                match modules.get_mut(label).unwrap() {
                    Module::Broadcast => {
                        for dest in dests {
                            queue.push_back((dest, pulse, label));
                        }
                    },
                    Module::FlipFlop(state) => {
                        if !pulse {
                            *state = !*state;
                            for dest in dests {
                                queue.push_back((dest, *state, label));
                            }
                        }
                    },
                    Module::Conjunction(memory) => {
                        memory.insert(from.to_string(), pulse);
                        let all_high = memory.values().all(|p| *p);
                        for dest in dests {
                            queue.push_back((dest, !all_high, label));
                        }
                    },
                }
            }
        }

        Some(pulses[0] * pulses[1])
    }

    pub fn part_b(&self) -> Option<i64> {
        None
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
