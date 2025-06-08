// tags: flood fill, pathfinding

use std::{collections::VecDeque, rc::Rc};

use crate::{
    direction::{self, DIRS},
    vector::{Vector, VectorMap, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    path: VectorMap<i64>,
    sample: bool,
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

    fn new(raw: Vec<Rc<str>>) -> Self {
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

        // Precalculate the distance to each point on the path
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
            path,
            sample: size.x < 50,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let offsets = direction::make_offset(2, 2);
        let cutoff = if self.sample { 1 } else { 100 };
        Some(self.cheat(&offsets, cutoff))
    }

    fn part_b(&self) -> Option<i64> {
        let offsets = direction::make_offset(1, 20);
        let cutoff = if self.sample { 50 } else { 100 };
        Some(self.cheat(&offsets, cutoff))
    }
}

impl Day {
    fn cheat(&self, offsets: &[Vector], cutoff: i64) -> i64 {
        let mut count = 0;
        // Since the path is linear, we can just get sum the distances from the start
        // to the cheat, and from the end of the cheat to the end of the path.
        for (pos, a) in self.path.iter() {
            for &dir in offsets {
                let Some(b) = self.path.get(pos + dir) else {
                    continue;
                };

                // The distance from the start of the path to the end gets canceled
                // out when doing path_len - (a + (path_len - b) + cheat_len)
                if b - a - dir.x.abs() - dir.y.abs() >= cutoff {
                    count += 1;
                }
            }
        }

        count
    }
}

crate::solution::test_solution!();
