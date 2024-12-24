use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    wires: HashMap<String, bool>,
    logic: HashMap<String, (String, Gate, String)>,
    sample: bool,
}

#[derive(Debug, Clone, Copy)]
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
        Self::solve(&mut wires, &self.logic, &HashMap::new());
        Some(Self::get_number(&wires, "z"))
    }

    fn part_b(&self) -> Option<String> {
        let num_swaps = if self.sample { 2 } else { 4 };
        let mut gates = HashSet::new();
        gates.extend(self.wires.keys());
        gates.extend(self.logic.keys());

        let mut final_swap: Option<HashMap<String, String>> = None;
        'combo: for combo in gates.iter().combinations(num_swaps * 2) {
            let mut wires = self.wires.clone();
            let mut swaps = HashMap::new();
            for swap in combo.chunks(2) {
                swaps.insert(swap[0].to_owned().clone(), swap[1].to_owned().clone());
                swaps.insert(swap[1].to_owned().clone(), swap[0].to_owned().clone());
            }

            let debug = swaps.contains_key("z00")
                && swaps.contains_key("z01")
                && swaps.contains_key("z02")
                && swaps.contains_key("z05");

            Self::solve(&mut wires, &self.logic, &swaps);
            let x = Self::get_number(&wires, "x");
            let y = Self::get_number(&wires, "y");
            let z = Self::get_number(&wires, "z");
            let check = if self.sample { x & y } else { x + y };
            if check == z {
                final_swap = Some(swaps);
                break 'combo;
            }
        }

        let wires = final_swap.unwrap().keys().sorted().join(",");
        Some(wires)
    }
}

impl Day {
    fn solve(
        wires: &mut HashMap<String, bool>,
        logic: &HashMap<String, (String, Gate, String)>,
        swaps: &HashMap<String, String>,
    ) {
        let mut unknowns = logic.clone();
        while !unknowns.is_empty() {
            let mut next_unknowns = HashMap::new();
            for (output, (a, gate, b)) in &unknowns {
                let a = swaps.get(a).unwrap_or(a);
                let b = swaps.get(b).unwrap_or(b);
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
}

crate::solution::test_solution!();
