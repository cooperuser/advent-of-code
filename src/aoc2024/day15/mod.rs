use std::collections::{HashSet, VecDeque};

use crate::{
    direction::Direction,
    vector::{Vector, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    inputs: Vec<Direction>,
    walls: VectorSet,
    boxes: VectorSet,
    size: Vector,
    start: Vector,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 10092,
            answer_b: 9021,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let (map, inputs) = raw.split_once(|line| line.is_empty()).unwrap();
        let size = Vector::new_usize(map[0].len(), map.len());
        let mut walls = VectorSet::new(size);
        let mut boxes = VectorSet::new(size);
        let mut start = None;

        for (y, line) in map.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Vector::new_usize(x, y);
                if c == '#' {
                    walls.insert(pos);
                } else if c == 'O' {
                    boxes.insert(pos);
                } else if c == '@' {
                    start = Some(pos);
                }
            }
        }

        Self {
            raw: raw.clone(),
            inputs: inputs.join("").chars().map(|c| c.into()).collect(),
            walls,
            boxes,
            size,
            start: start.unwrap(),
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut boxes = self.boxes.clone();
        let mut pos = self.start;
        'input: for &input in &self.inputs {
            let mut next = pos + input;
            if self.walls.contains(next) {
                continue;
            }
            if boxes.contains(next) {
                while boxes.contains(next) {
                    next += input;
                    if self.walls.contains(next) {
                        continue 'input;
                    }
                }
                boxes.remove(pos + input);
                boxes.insert(next);
            }
            pos += input;
        }

        Some(boxes.iter().map(|b| 100 * b.y + b.x).sum())
    }

    fn part_b(&self) -> Option<i64> {
        let size = Vector::new(self.size.x * 2, self.size.y);
        let mut walls = VectorSet::new(size);
        let mut boxes = VectorSet::new(size);
        let mut pos = Vector::new(self.start.x * 2, self.start.y);

        for wall in self.walls.iter() {
            walls.insert(Vector::new(wall.x * 2, wall.y));
            walls.insert(Vector::new(wall.x * 2 + 1, wall.y));
        }

        for b in self.boxes.iter() {
            boxes.insert(Vector::new(b.x * 2, b.y));
        }

        'input: for &input in &self.inputs {
            if walls.contains(pos + input) {
                continue;
            }
            match input {
                Direction::North | Direction::South => {
                    for next in [pos + input, pos + input + Direction::West] {
                        if boxes.contains(next) {
                            let mut block: HashSet<Vector> = HashSet::from([next]);
                            let mut deque: VecDeque<Vector> = VecDeque::from([next]);
                            while let Some(spot) = deque.pop_front() {
                                let center = spot + input;
                                let right = center + Direction::East;
                                if walls.contains(center) || walls.contains(right) {
                                    continue 'input;
                                }
                                for check in [spot + input + Direction::West, center, right] {
                                    if boxes.contains(check) {
                                        block.insert(check);
                                        deque.push_back(check);
                                    }
                                }
                            }

                            for &b in &block {
                                boxes.remove(b);
                            }
                            for &b in &block {
                                boxes.insert(b + input);
                            }
                        }
                    }
                }
                Direction::East | Direction::West => {
                    let mut next = pos + input;
                    if input == Direction::West {
                        next += input
                    }
                    if boxes.contains(next) {
                        let mut block: Vec<Vector> = Vec::from([next]);
                        while boxes.contains(next) {
                            next = next + input + input;
                            if boxes.contains(next) {
                                block.push(next);
                            }

                            let check = if input == Direction::East {
                                next
                            } else {
                                next - input
                            };

                            if walls.contains(check) {
                                continue 'input;
                            }
                        }

                        for b in block {
                            boxes.remove(b);
                            boxes.insert(b + input);
                        }
                    }
                }
            }
            pos += input;
        }

        Some(boxes.iter().map(|b| 100 * b.y + b.x).sum())
    }
}

crate::solution::test_solution!();
