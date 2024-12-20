// tags: caching, flood fill

use std::collections::VecDeque;

use crate::{
    direction::{self, DIRS},
    vector::{Vector, VectorMap, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    grid: VectorSet,
    size: Vector,
    start: Vector,
    end: Vector,
    path: VectorMap<i64>,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 44,
            answer_b: 285,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut grid = VectorSet::new(size);
        let mut start: Option<Vector> = None;
        let mut end: Option<Vector> = None;
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Vector::new_usize(x, y);
                if c == 'S' {
                    start = Some(pos);
                } else if c == 'E' {
                    end = Some(pos);
                } else if c == '#' {
                    continue;
                }
                grid.insert(pos);
            }
        }

        let mut path: VectorMap<i64> = VectorMap::new(size);
        let mut deque: VecDeque<(Vector, i64)> = VecDeque::from([(start.unwrap(), 0)]);
        while let Some((pos, distance)) = deque.pop_front() {
            if !grid.contains(pos) || path.contains(pos) {
                continue;
            }
            path.insert(pos, distance);

            if pos == end.unwrap() {
                break;
            }

            for dir in DIRS {
                deque.push_back((pos + dir, distance + 1));
            }
        }

        Self {
            raw: raw.clone(),
            grid,
            size,
            start: start.unwrap(),
            end: end.unwrap(),
            path,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let offsets = direction::make_offset(2, 2);
        let cutoff = if self.size.x < 50 { 1 } else { 100 };
        Some(self.cheat(&offsets, cutoff))
    }

    fn part_b(&self) -> Option<i64> {
        let offsets = direction::make_offset(1, 20);
        let cutoff = if self.size.x < 50 { 50 } else { 100 };
        Some(self.cheat(&offsets, cutoff))
    }
}

impl Day {
    fn cheat(&self, offsets: &[Vector], cutoff: i64) -> i64 {
        let mut count = 0;
        let max = self.get_path(self.start, self.end).unwrap();
        let mut cache: VectorMap<i64> = VectorMap::new(self.size);
        for pos in self.grid.iter() {
            let a = self.get_path(self.start, pos).unwrap();
            for &dir in offsets {
                let pos = pos + dir;
                if !self.grid.contains(pos) {
                    continue;
                }

                let b = match cache.get(pos) {
                    Some(b) => b,
                    None => {
                        let b = self.get_path(pos, self.end).unwrap();
                        cache.insert(pos, b);
                        b
                    }
                };

                let dist = a + b + dir.x.abs() + dir.y.abs();
                if max - dist >= cutoff {
                    count += 1;
                }
            }
        }

        count
    }

    fn get_path(&self, start: Vector, end: Vector) -> Option<i64> {
        Some(self.path.get(end)? - self.path.get(start)?)
    }
}

crate::solution::test_solution!();
