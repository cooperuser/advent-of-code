#![allow(dead_code)]

use std::collections::HashMap;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 1320;
pub const ANSWER_B: i64 = 145;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    pieces: Vec<String>,
    operations: Vec<Operation>,
}

#[derive(Debug)]
enum Operation {
    Dash(String),
    Equals(String, i64),
}

struct Lens {
    label: String,
    value: i64,
}

fn hash(piece: &str) -> i64 {
    let mut value = 0;
    for ch in piece.chars() {
        value += ch as i64;
        value *= 17;
        value %= 256;
    }
    value
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut ops = Vec::new();
        for op in raw[0].split(',') {
            if op.chars().last().unwrap() == '-' {
                ops.push(Operation::Dash(op[0..op.len() - 1].to_string()));
            } else {
                let (label, value) = op.split_once('=').unwrap();
                let value: i64 = value.parse().unwrap();
                ops.push(Operation::Equals(label.to_string(), value));
            }
        }
        Self {
            raw: raw.clone(),
            pieces: raw[0].split(',').map(|s| s.to_string()).collect(),
            operations: ops,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut sum = 0;

        for piece in &self.pieces {
            sum += hash(piece);
        }

        Some(sum)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut boxes: HashMap<i64, Vec<Lens>> = HashMap::new();
        for op in &self.operations {
            match op {
                Operation::Dash(label) => {
                    let id = hash(label);
                    let boxx = boxes.entry(id).or_insert(vec![]);
                    if let Some((index, _)) = boxx.iter().enumerate().find(|(_, lens)| lens.label == *label) {
                        boxx.remove(index);
                    }
                },
                Operation::Equals(label, value) => {
                    let id = hash(label);
                    let boxx = boxes.entry(id).or_insert(vec![]);
                    if let Some((index, _)) = boxx.iter().enumerate().find(|(_, lens)| lens.label == *label) {
                        boxx[index].value = *value;
                    } else {
                        boxx.push(Lens { label: label.clone(), value: *value });
                    }
                },
            }
        }

        let mut sum = 0;
        for (id, boxx) in &boxes {
            for (lens_id, lens) in boxx.iter().enumerate() {
                let mut value = id + 1;
                value *= (lens_id + 1) as i64;
                value *= lens.value;
                sum += value;
            }
        }
        Some(sum)
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
