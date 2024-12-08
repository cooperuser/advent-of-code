use std::collections::HashMap;

use crate::vector::{Vector, VectorSet};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    antennae: HashMap<char, Vec<Vector>>,
    size: Vector,
}

impl crate::solution::Solution<i64> for Day {
    fn meta() -> crate::solution::Meta<i64> {
        crate::solution::Meta::<i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 14,
            answer_b: 34,
        }
    }

    fn new(raw: Vec<String>) -> Self {
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
        for spaces in self.antennae.values() {
            for (i, &first) in spaces.iter().enumerate() {
                for &second in spaces.iter().skip(i + 1) {
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
        for spaces in self.antennae.values() {
            for (i, &first) in spaces.iter().enumerate() {
                for &second in spaces.iter().skip(i + 1) {
                    let diff = second - first;
                    let mut a = first;
                    let mut b = second;
                    while a.contained_in(Vector::zero(), self.size) {
                        antinodes.insert(a);
                        a -= diff;
                    }
                    while b.contained_in(Vector::zero(), self.size) {
                        antinodes.insert(b);
                        b += diff;
                    }
                }
            }
        }
        Some(antinodes.len() as i64)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::solution::Solution;

    #[test]
    fn part_a() {
        let meta = Day::meta();
        let solution = Day::new(crate::split(meta.sample_a));
        assert_eq!(solution.part_a(), Some(meta.answer_a));
    }

    #[test]
    fn part_b() {
        let meta = Day::meta();
        let solution = Day::new(crate::split(meta.sample_b));
        assert_eq!(solution.part_b(), Some(meta.answer_b));
    }
}
