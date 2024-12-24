use std::collections::HashMap;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    wires: HashMap<String, bool>,
    logic: HashMap<String, (String, Gate, String)>,
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
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut wires = self.wires.clone();
        let mut unknowns = self.logic.clone();
        while !unknowns.is_empty() {
            let mut next_unknowns = HashMap::new();
            for (output, (a, gate, b)) in &unknowns {
                if let (Some(&a), Some(&b)) = (wires.get(a), wires.get(b)) {
                    wires.insert(
                        output.clone(),
                        match gate {
                            Gate::And => a && b,
                            Gate::Or => a || b,
                            Gate::Xor => a ^ b,
                        },
                    );
                } else {
                    next_unknowns.insert(output.clone(), (a.clone(), *gate, b.clone()));
                }
            }
            unknowns = next_unknowns;
        }

        let mut z: Vec<_> = wires.keys().filter(|s| s.starts_with("z")).collect();
        z.sort();
        z.reverse();

        let mut output = 0;
        for z in z {
            output <<= 1;
            if *wires.get(z).unwrap() {
                output |= 1;
            }
        }

        Some(output)
    }

    fn part_b(&self) -> Option<String> {
        None
    }
}

crate::solution::test_solution!();
