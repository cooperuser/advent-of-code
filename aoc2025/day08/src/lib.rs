use std::collections::HashMap;

use utils::{disjointset::DisjointSet, prelude::*, vector3::Vector3};

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
        let mut set = DisjointSet::new(length);

        for &((a, b), _) in self.distances.iter().take(connections) {
            set.union(a, b);
        }

        let mut sizes = set.sizes();
        sizes.sort();

        Some(sizes.iter().rev().take(3).map(|&n| n as i64).product())
    }

    fn part_b(&self) -> Option<i64> {
        let length = self.boxes.len();
        let mut set = DisjointSet::new(length);
        let mut count = 0;

        for &((a, b), _) in self.distances.iter() {
            if set.union(a, b) == length {
                break;
            }
            count += 1;
        }

        let (a, b) = self.distances[count].0;
        let a = self.boxes[a];
        let b = self.boxes[b];

        Some(a.x * b.x)
    }
}

utils::solution::test_solution!(aoc2025, day08);
