#![allow(dead_code)]

use std::collections::HashMap;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: f64 = 152.0;
pub const SAMPLE_B: f64 = 301.0;

#[derive(Debug, Clone)]
enum Monkey {
    Op(String, String, char),
    Num(f64),
    Unknown
}

impl Monkey {
    fn solve(&self, monkeys: &HashMap<String, Monkey>) -> f64 {
        match self {
            Monkey::Num(num) => *num,
            Monkey::Op(a, b, op) => {
                let a = monkeys.get(a).unwrap();
                let b = monkeys.get(b).unwrap();
                match op {
                    '+' => a.solve(monkeys) + b.solve(monkeys),
                    '-' => a.solve(monkeys) - b.solve(monkeys),
                    '*' => a.solve(monkeys) * b.solve(monkeys),
                    '/' => a.solve(monkeys) / b.solve(monkeys),
                    _ => panic!()
                }
            }
            _ => panic!()
        }
    }

    fn solve_unknown(&self, monkeys: &HashMap<String, Monkey>) -> (f64, f64) {
        match self {
            Monkey::Num(num) => (0.0, *num),
            Monkey::Unknown => (1.0, 0.0),
            Monkey::Op(a, b, op) => {
                let a = monkeys.get(a).unwrap().solve_unknown(monkeys);
                let b = monkeys.get(b).unwrap().solve_unknown(monkeys);
                match op {
                    '+' => (a.0 + b.0, a.1 + b.1),
                    '-' => (a.0 - b.0, a.1 - b.1),
                    '*' => if b.0 == 0.0 {
                        (a.0 * b.1, a.1 * b.1)
                    } else if a.0 == 0.0 {
                        (b.0 * a.1, b.1 * a.1)
                    } else {
                        panic!("too deep")
                    },
                    '/' => (a.0 / b.1, a.1 / b.1),
                    _ => panic!()
                }
            }
        }
    }
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    monkeys: HashMap<String, Monkey>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut monkeys = HashMap::new();
        for line in &raw {
            if line.is_empty() { continue }
            let words: Vec<&str> = line.split_whitespace().collect();
            let name = words[0].split_once(':').unwrap().0.to_string();
            let monkey = if words.len() == 2 {
                Monkey::Num(words[1].parse().unwrap())
            } else {
                Monkey::Op(
                    words[1].to_string(),
                    words[3].to_string(),
                    words[2].chars().next().unwrap()
                )
            };
            monkeys.insert(name, monkey);
        }
        Self {
            raw: raw.clone(),
            monkeys,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> f64 {
        self.monkeys.get("root").unwrap().solve(&self.monkeys)
    }

    pub fn part_b(&self) -> f64 {
        let mut monkeys = self.monkeys.clone();
        monkeys.insert("humn".to_string(), Monkey::Unknown);
        if let Monkey::Op(a, b, _) = monkeys.get("root").unwrap() {
            let a = monkeys.get(a).unwrap();
            let b = monkeys.get(b).unwrap();
            let a = a.solve_unknown(&monkeys);
            let b = b.solve_unknown(&monkeys);
            return ((b.1 - a.1) / a.0).round()
        }
        0.0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_a(), SAMPLE_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_b(), SAMPLE_B);
    }
}
