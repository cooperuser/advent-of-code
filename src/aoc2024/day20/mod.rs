use std::collections::{HashSet, VecDeque};

use crate::{
    direction::DIRS,
    vector::{Vector, VectorSet},
};

const DOUBLE_DIRS: [Vector; 8] = [
    Vector::new(-2, 0),
    Vector::new(-1, -1),
    Vector::new(-1, 1),
    Vector::new(0, -2),
    Vector::new(0, 2),
    Vector::new(1, -1),
    Vector::new(1, 1),
    Vector::new(2, 0),
];

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    grid: VectorSet,
    size: Vector,
    start: Vector,
    end: Vector,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 44,
            answer_b: 0,
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

        Self {
            raw: raw.clone(),
            grid,
            size,
            start: start.unwrap(),
            end: end.unwrap(),
        }
    }

    fn part_a(&self) -> Option<i64> {
        let cutoff = if self.size.x < 50 { 1 } else { 100 };
        let path = self.get_path(self.start, self.end).unwrap();
        let max = path;
        let len = self.grid.len();
        let mut count = 0;
        for (i, pos) in self.grid.iter().enumerate() {
            let Some(a) = self.get_path(self.start, pos) else {
                continue;
            };
            for dir in DOUBLE_DIRS {
                if !self.grid.contains(pos + dir) {
                    continue;
                }
                let Some(b) = self.get_path(pos + dir, self.end) else {
                    continue;
                };
                let dist = a + b + 2;
                if max - dist >= cutoff {
                    count += 1;
                }
            }
            println!("{:?} / {:?}", i, len);
        }
        Some(count)
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

impl Day {
    fn get_path(&self, start: Vector, end: Vector) -> Option<i64> {
        let mut deque: VecDeque<(Vector, i64)> = VecDeque::from([(start, 0)]);
        let mut visited = VectorSet::new(self.size);
        while let Some((pos, distance)) = deque.pop_front() {
            if !pos.contained_in(Vector::zero(), self.size)
                || !visited.insert(pos).unwrap()
                || !self.grid.contains(pos)
            {
                continue;
            }

            if pos == end {
                return Some(distance);
            }

            for dir in DIRS {
                deque.push_back((pos + dir, distance + 1));
            }
        }

        None
    }

    fn cheat(&self, path: &[Vector], index: usize) -> Option<i64> {
        let mut visited = VectorSet::new(self.size);
        // for &pos in path.iter().take(index) {
        //     visited.insert(pos);
        // }

        let mut deque: VecDeque<(Vector, usize)> = VecDeque::new();
        let start = path[index];
        for dir in DOUBLE_DIRS {
            deque.push_back((start + dir, index + 2));
        }

        while let Some((pos, distance)) = deque.pop_front() {
            if !pos.contained_in(Vector::zero(), self.size)
                || !visited.insert(pos).unwrap()
                || !self.grid.contains(pos)
            {
                continue;
            }

            if pos == self.end {
                return Some(distance as i64);
            }

            for dir in DIRS {
                deque.push_back((pos + dir, distance + 1));
            }
        }

        None
    }

    // fn path(&self, start: Vector, cheated: bool) -> Vec<i64> {
    //     let mut deque: VecDeque<(Vector, bool, i64, VectorSet)> =
    //         VecDeque::from([(start, cheated, 0, VectorSet::new(self.size))]);
    //     let mut distances = Vec::new();
    //     while let Some((pos, cheated, distance, mut path)) = deque.pop_front() {
    //         if !pos.contained_in(Vector::zero(), self.size)
    //             || !path.insert(pos).unwrap()
    //             || !self.grid.contains(pos)
    //         {
    //             continue;
    //         }
    //
    //         if pos == self.end {
    //             distances.push(distance);
    //         }
    //
    //         for dir in DIRS {
    //             deque.push_back((pos + dir, cheated, distance + 1, path.clone()));
    //         }
    //
    //         if !cheated {
    //             for dir in DOUBLE_DIRS {
    //                 deque.push_back((pos + dir, true, distance + 2, path.clone()));
    //             }
    //         }
    //     }
    //
    //     distances
    // }
}

crate::solution::test_solution!();
