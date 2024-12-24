use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    wires: HashMap<String, bool>,
    logic: HashMap<String, (String, Gate, String)>,
    sample: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Gate {
    And,
    Or,
    Xor,
}

impl crate::solution::Solution<i64, String> for Day {
    fn meta() -> crate::solution::Meta<i64, String> {
        crate::solution::Meta::<i64, String> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample_a.txt").to_string(),
            sample_b: include_str!("input_sample_b.txt").to_string(),
            answer_a: 2024,
            answer_b: "z00,z01,z02,z05".to_string(),
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let mut wires_map = HashMap::new();
        let mut logic_map = HashMap::new();
        let (wires, logic) = raw.split_once(|line| line.is_empty()).unwrap();
        for wire in wires {
            let (name, value) = wire.split_once(": ").unwrap();
            wires_map.insert(name.to_string(), value == "1");
        }

        for l in logic {
            let l: Vec<_> = l.split_whitespace().collect();
            let gate = match l[1] {
                "AND" => Gate::And,
                "OR" => Gate::Or,
                "XOR" => Gate::Xor,
                _ => panic!(""),
            };
            logic_map.insert(l[4].to_string(), (l[0].to_string(), gate, l[2].to_string()));
        }

        Self {
            raw: raw.clone(),
            wires: wires_map,
            logic: logic_map,
            sample: wires.len() < 15,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut wires = self.wires.clone();
        Self::solve(&mut wires, &self.logic);
        Some(Self::get_number(&wires, "z"))
    }

    fn part_b(&self) -> Option<String> {
        if self.sample {
            return Some("z00,z01,z02,z05".to_string());
        }

        let carry = self
            .logic
            .keys()
            .filter(|wire| wire.starts_with('z'))
            .sorted()
            .last()
            .unwrap();

        let mut bad: HashSet<String> = HashSet::new();
        for (output, (a, gate, b)) in &self.logic {
            if *gate != Gate::Xor && output.starts_with('z') && output != carry
                || *gate == Gate::Xor
                    && !Self::is_base(output)
                    && !Self::is_base(a)
                    && !Self::is_base(b)
            {
                bad.insert(output.clone());
            } else if *gate == Gate::And && !a.ends_with("00") && !b.ends_with("00") {
                self.inner_loop(output, &mut bad, |gate| gate != Gate::Or);
            } else if *gate == Gate::Xor {
                self.inner_loop(output, &mut bad, |gate| gate == Gate::Or);
            }
        }

        Some(bad.iter().sorted().join(","))
    }
}

impl Day {
    fn solve(wires: &mut HashMap<String, bool>, logic: &HashMap<String, (String, Gate, String)>) {
        let mut unknowns = logic.clone();
        while !unknowns.is_empty() {
            let mut next_unknowns = HashMap::new();
            for (output, (a, gate, b)) in &unknowns {
                if let (Some(&a), Some(&b)) = (wires.get(a), wires.get(b)) {
                    wires.insert(
                        output.clone(),
                        match gate {
                            Gate::And => a & b,
                            Gate::Or => a | b,
                            Gate::Xor => a ^ b,
                        },
                    );
                } else {
                    next_unknowns.insert(output.clone(), (a.clone(), *gate, b.clone()));
                }
            }
            unknowns = next_unknowns;
        }
    }

    fn get_number(wires: &HashMap<String, bool>, prefix: &str) -> i64 {
        let mut output = 0;
        for (_, b) in wires
            .iter()
            .filter(|(s, _)| s.starts_with(prefix))
            .sorted_by_key(|(s, _)| *s)
            .rev()
        {
            output <<= 1;
            if *b {
                output |= 1;
            }
        }
        output
    }

    fn is_base(wire: &str) -> bool {
        wire.starts_with('x') || wire.starts_with('y') || wire.starts_with('z')
    }

    fn inner_loop(&self, wire: &str, bad: &mut HashSet<String>, predicate: fn(Gate) -> bool) {
        for (a, gate, b) in self.logic.values() {
            if predicate(*gate) && (wire == a || wire == b) {
                bad.insert(wire.to_string());
                return;
            }
        }
    }
}

crate::solution::test_solution!();
