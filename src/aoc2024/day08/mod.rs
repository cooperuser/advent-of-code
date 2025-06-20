use crate::prelude::*;
use std::collections::HashMap;

use crate::vector::{Vector, VectorSet};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    antennae: HashMap<char, Vec<Vector>>,
    size: Vector,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 14,
            answer_b: 34,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut antennae = HashMap::new();
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '.' {
                    continue;
                }
                antennae
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push(Vector::new_usize(x, y));
            }
        }

        Self {
            raw: raw.clone(),
            antennae,
            size,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut antinodes = VectorSet::new(self.size);
        for antenna in self.antennae.values() {
            for (i, &first) in antenna.iter().enumerate() {
                for &second in antenna.iter().skip(i + 1) {
                    let diff = second - first;
                    antinodes.insert(first - diff);
                    antinodes.insert(second + diff);
                }
            }
        }
        Some(antinodes.len() as i64)
    }

    fn part_b(&self) -> Option<i64> {
        let mut antinodes = VectorSet::new(self.size);
        for antenna in self.antennae.values() {
            for (i, &first) in antenna.iter().enumerate() {
                for &second in antenna.iter().skip(i + 1) {
                    let diff = second - first;
                    let mut node = first;
                    while node.contained_in(Vector::zero(), self.size) {
                        antinodes.insert(node);
                        node -= diff;
                    }
                    let mut node = second;
                    while node.contained_in(Vector::zero(), self.size) {
                        antinodes.insert(node);
                        node += diff;
                    }
                }
            }
        }
        Some(antinodes.len() as i64)
    }
}

crate::solution::test_solution!();
