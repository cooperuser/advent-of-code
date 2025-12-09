use std::collections::HashMap;

use utils::{prelude::*, vector3::Vector3};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    boxes: Vec<Vector3>,
    distances: Vec<((usize, usize), i64)>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 40,
            answer_b: 25272,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut boxes = Vec::new();
        for line in &raw {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            let z = parts.next().unwrap().parse().unwrap();
            boxes.push(Vector3::new(x, y, z));
        }

        let mut map: HashMap<(usize, usize), i64> = HashMap::new();
        for (i, &a) in boxes[..boxes.len() - 1].iter().enumerate() {
            for (j, &b) in boxes[i + 1..].iter().enumerate() {
                let distance = (b - a).sqr_distance();
                map.insert((i, i + j + 1), distance);
            }
        }

        let mut distances: Vec<((usize, usize), i64)> = map.into_iter().collect();
        distances.sort_by_key(|&(_, distance)| distance);

        Self {
            raw,
            boxes,
            distances,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let length = self.boxes.len();
        let connections = if length < 30 { 10 } else { 1000 };

        let mut labels: Vec<_> = (0..length).collect();
        for &((a, b), _) in self.distances.iter().take(connections) {
            let a = labels[a];
            let b = labels[b];
            if a == b {
                continue;
            }

            let lowest = a.min(b);
            labels = labels
                .iter()
                .map(|&label| match label == a || label == b {
                    true => lowest,
                    false => label,
                })
                .collect();
        }

        let mut map: HashMap<usize, i64> = HashMap::new();
        for &label in &labels {
            *map.entry(label).or_default() += 1;
        }

        let mut values: Vec<_> = map.values().cloned().collect();
        values.sort();
        values.reverse();

        Some(values.iter().take(3).product())
    }

    fn part_b(&self) -> Option<i64> {
        let mut labels: Vec<_> = (0..self.boxes.len()).collect();
        let mut count = 0;

        for &((a, b), _) in &self.distances {
            count += 1;

            let a = labels[a];
            let b = labels[b];
            if a == b {
                continue;
            }

            let lowest = a.min(b);
            let mut single = true;
            labels = labels
                .iter()
                .map(|&label| match label == a || label == b {
                    true => lowest,
                    false => {
                        single = false;
                        label
                    }
                })
                .collect();

            if single {
                break;
            }
        }

        let (a, b) = self.distances[count - 1].0;
        let a = self.boxes[a];
        let b = self.boxes[b];

        Some(a.x * b.x)
    }
}

utils::solution::test_solution!(aoc2025, day08);
