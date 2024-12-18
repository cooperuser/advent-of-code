use std::collections::VecDeque;

use crate::{
    direction::DIRS,
    vector::{Vector, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    bytes: Vec<Vector>,
    size: Vector,
    end: Vector,
}

impl crate::solution::Solution<String> for Day {
    fn meta() -> crate::solution::Meta<String> {
        crate::solution::Meta::<String> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: "22".to_string(),
            answer_b: "6,1".to_string(),
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let size = if raw.len() < 50 {
            Vector::new(7, 7)
        } else {
            Vector::new(71, 71)
        };
        let mut bytes = Vec::new();
        for line in raw.iter() {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            bytes.push(Vector::new(x, y));
        }
        Self {
            raw: raw.clone(),
            bytes,
            size,
            end: size - Vector::new(1, 1),
        }
    }

    fn part_a(&self) -> Option<String> {
        let mut map = VectorSet::new(self.size);
        for byte in 0..if self.bytes.len() < 50 { 12 } else { 1024 } {
            map.insert(self.bytes[byte]);
        }

        self.search(&map).map(|n| n.to_string())
    }

    fn part_b(&self) -> Option<String> {
        let mut map = VectorSet::new(self.size);
        let mut i = 0;
        while self.search(&map).is_some() {
            map.insert(self.bytes[i]);
            i += 1;
        }

        let byte = self.bytes[i - 1];
        Some(format!("{},{}", byte.x, byte.y))
    }
}

impl Day {
    fn search(&self, map: &VectorSet) -> Option<i64> {
        let mut visited = VectorSet::new(self.size);
        let mut deque: VecDeque<(Vector, i64)> = VecDeque::from([(Vector::zero(), 0)]);
        while let Some((pos, distance)) = deque.pop_front() {
            if !pos.contained_in(Vector::zero(), self.size) || !visited.insert(pos).unwrap() {
                continue;
            } else if pos == self.end {
                return Some(distance);
            }
            for dir in DIRS {
                let next = pos + dir;
                if !map.contains(next) {
                    deque.push_back((pos + dir, distance + 1));
                }
            }
        }

        None
    }
}

crate::solution::test_solution!();
