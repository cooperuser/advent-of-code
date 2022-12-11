#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 10605;
pub const SAMPLE_B: i64 = 2713310158;

#[derive(Clone, Debug)]
enum Operation {
    Add,
    Mul
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        match s {
            "+" => Self::Add,
            "*" => Self::Mul,
            _ => panic!()
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: (Operation, i64),
    test_mod: i64,
    targets: (usize, usize)
}

impl Monkey {
    fn apply_operation(&self, value: i64) -> i64 {
        match self.operation.0 {
            Operation::Add => match self.operation.1 {
                0 => value + value,
                num => value + num,
            },
            Operation::Mul => match self.operation.1 {
                0 => value * value,
                num => value * num,
            },
        }
    }
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    monkeys: Vec<Monkey>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut monkeys = vec![];

        for chunk in raw.split(|line| line == "") {
            if chunk.is_empty() { continue }
            let chunk: Vec<Vec<&str>> = chunk
                .iter()
                .map(|line| line.trim().split_whitespace().collect::<Vec<&str>>())
                .collect();

            let items: Vec<i64> = chunk[1][2..].join("").split(",").map(|num| num.parse::<i64>().unwrap()).collect();

            monkeys.push(Monkey {
                items,
                operation: (Operation::from(chunk[2][4]), chunk[2][5].parse().unwrap_or_default()),
                test_mod: chunk[3][3].parse().unwrap(),
                targets: (chunk[4][5].parse().unwrap(), chunk[5][5].parse().unwrap())
            });
        }

        Self {
            raw: raw.clone(),
            monkeys,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let rounds = 20;
        let mut monkeys = self.monkeys.clone();
        let mut items_checked = vec![0; self.monkeys.len()];

        for _ in 0..rounds {
            for m in 0..monkeys.len() {
                let mut targets: Vec<Vec<i64>> = vec![vec![]; monkeys.len()];
                let monkey = monkeys.get_mut(m).unwrap();
                while !monkey.items.is_empty() {
                    items_checked[m] += 1;
                    let item = monkey.items.remove(0);
                    let item = monkey.apply_operation(item);
                    let item = item / 3;
                    if item % monkey.test_mod == 0 {
                        targets[monkey.targets.0].push(item);
                    } else {
                        targets[monkey.targets.1].push(item);
                    }
                }
                for t in 0..targets.len() {
                    monkeys[t].items.append(&mut targets[t]);
                }
            }
        }
        items_checked.sort();
        items_checked[items_checked.len() - 1] * items_checked[items_checked.len() - 2]
    }

    pub fn part_b(&self) -> i64 {
        let rounds = 10000;
        let mut monkeys = self.monkeys.clone();
        let mut items_checked = vec![0; self.monkeys.len()];

        let modulo = monkeys.iter().fold(1, |a, b| a * b.test_mod);

        for _ in 0..rounds {
            for m in 0..monkeys.len() {
                let mut targets: Vec<Vec<i64>> = vec![vec![]; monkeys.len()];
                let monkey = monkeys.get_mut(m).unwrap();
                while !monkey.items.is_empty() {
                    items_checked[m] += 1;
                    let item = monkey.items.remove(0);
                    let item = monkey.apply_operation(item);
                    let item = item % modulo;
                    if item % monkey.test_mod == 0 {
                        targets[monkey.targets.0].push(item);
                    } else {
                        targets[monkey.targets.1].push(item);
                    }
                }
                for t in 0..targets.len() {
                    monkeys[t].items.append(&mut targets[t]);
                }
            }
        }
        items_checked.sort();
        items_checked[items_checked.len() - 1] * items_checked[items_checked.len() - 2]
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
