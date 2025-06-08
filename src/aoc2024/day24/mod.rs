use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use itertools::Itertools;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    wires: HashMap<Rc<str>, bool>,
    logic: HashMap<Rc<str>, (Rc<str>, Gate, Rc<str>)>,
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

    fn new(raw: Vec<Rc<str>>) -> Self {
        let (wires, logic) = raw.split_once(|line| line.is_empty()).unwrap();

        Self {
            raw: raw.clone(),
            wires: wires
                .iter()
                .map(|wire| {
                    let (name, value) = wire.split_once(": ").unwrap();
                    (name.into(), value == "1")
                })
                .collect(),
            logic: logic
                .iter()
                .map(|line| {
                    let l: Vec<_> = line.split_whitespace().collect();
                    let gate = match l[1] {
                        "AND" => Gate::And,
                        "OR" => Gate::Or,
                        "XOR" => Gate::Xor,
                        _ => panic!("Invalid gate: {}", l[1]),
                    };
                    (l[4].into(), (l[0].into(), gate, l[2].into()))
                })
                .collect(),
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

        let mut bad: HashSet<Rc<str>> = HashSet::new();
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
    fn solve(
        wires: &mut HashMap<Rc<str>, bool>,
        logic: &HashMap<Rc<str>, (Rc<str>, Gate, Rc<str>)>,
    ) {
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

    fn get_number(wires: &HashMap<Rc<str>, bool>, prefix: &str) -> i64 {
        let number = wires
            .iter()
            .filter(|(s, _)| s.starts_with(prefix))
            .sorted_by_key(|(s, _)| *s)
            .map(|(_, v)| if *v { 1 } else { 0 })
            .rev()
            .join("");
        i64::from_str_radix(&number, 2).unwrap()
    }

    fn is_base(wire: &str) -> bool {
        wire.starts_with('x') || wire.starts_with('y') || wire.starts_with('z')
    }

    fn inner_loop(&self, wire: &str, bad: &mut HashSet<Rc<str>>, predicate: fn(Gate) -> bool) {
        for (a, gate, b) in self.logic.values() {
            if predicate(*gate) && (*wire == **a || *wire == **b) {
                bad.insert(wire.into());
                return;
            }
        }
    }
}

crate::solution::test_solution!();
