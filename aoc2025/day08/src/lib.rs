use std::collections::{HashMap, HashSet};

use utils::{prelude::*, vector3::Vector3};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    boxes: Vec<Vector3>,
    distances: HashMap<(usize, usize), i64>,
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

        let mut distances: HashMap<(usize, usize), i64> = HashMap::new();
        for (i, &a) in boxes[..boxes.len() - 1].iter().enumerate() {
            for (j, &b) in boxes[i + 1..].iter().enumerate() {
                let distance = (b - a).sqr_distance();
                distances.insert((i, i + j + 1), distance);
            }
        }

        Self {
            raw,
            boxes,
            distances,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let length = self.boxes.len();
        let connections = if length < 30 { 10 } else { 1000 };

        let mut distances: Vec<_> = self.distances.iter().collect();
        distances.sort_by_key(|a| a.1);

        let mut labels: Vec<_> = (0..length).collect();
        for (a, b) in distances.iter().map(|&(&pair, _)| pair).take(connections) {
            let label_a = labels[a];
            let label_b = labels[b];
            if label_a == label_b {
                continue;
            }

            let lowest = label_a.min(label_b);
            labels = labels
                .iter()
                .map(|&label| {
                    if label == label_a || label == label_b {
                        lowest
                    } else {
                        label
                    }
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
        let mut distances: Vec<_> = self.distances.iter().collect();
        distances.sort_by_key(|a| a.1);

        let mut seen: HashSet<usize> = HashSet::new();
        let mut count = 0;
        for (a, b) in distances.iter().map(|&(&pair, _)| pair) {
            seen.insert(a);
            seen.insert(b);
            if seen.len() == self.boxes.len() {
                break;
            }
            count += 1;
        }

        let &(a, b) = distances[count].0;
        let a = self.boxes[a];
        let b = self.boxes[b];

        Some(a.x * b.x)
    }
}

fn print_graph(graph: &Vec<Vec<bool>>) {
    let mut strings = Vec::new();
    for y in graph {
        let mut s = Vec::new();
        for x in y {
            if *x {
                s.push("x");
            } else {
                s.push(".");
            }
        }
        println!("{}", s.join(" "));
        strings.push(s.join(" "));
    }
}

utils::solution::test_solution!(aoc2025, day08);
