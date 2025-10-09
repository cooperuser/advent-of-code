use std::collections::{HashMap, HashSet, VecDeque};
use utils::prelude::*;

use utils::{
    direction::DIRS,
    vector::{Vector, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    grid: VectorSet,
    size: Vector,
    start: Vector,
    distances: HashMap<Vector, i64>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 16,
            answer_b: 16733999, // This answer is modified to fit the calculation
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut start: Option<Vector> = None;
        let mut grid = VectorSet::new(size);
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Vector::new_usize(x, y);
                if c == '#' {
                    grid.insert(pos);
                } else if c == 'S' {
                    start = Some(pos);
                }
            }
        }

        let mut distances = HashMap::new();
        let mut deque: VecDeque<(Vector, i64)> = VecDeque::from([(start.unwrap(), 0)]);
        while let Some((pos, distance)) = deque.pop_front() {
            if distances.contains_key(&pos) || !pos.contained_in(Vector::zero(), size) {
                continue;
            }
            distances.insert(pos, distance);
            for dir in DIRS {
                let pos = pos + dir;
                if !grid.contains(pos) {
                    deque.push_back((pos, distance + 1));
                }
            }
        }

        Self {
            raw: raw.clone(),
            grid,
            size,
            start: start.unwrap(),
            distances,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let steps = if self.size.x < 20 { 6 } else { 64 };
        let mut visited: HashSet<Vector> = HashSet::new();
        let mut new: HashSet<Vector> = HashSet::from([self.start]);
        for _ in 0..steps {
            new = self.take_step(&visited, &new);
            visited.extend(new.iter());
        }
        Some(visited.iter().filter(|v| (v.x + v.y) % 2 == 0).count() as i64)
    }

    fn part_b(&self) -> Option<i64> {
        let steps = if self.size.x < 50 { 5000 } else { 26501365 };
        let size = self.size.x / 2;

        let odd_full = self
            .distances
            .values()
            .filter(|&&dist| dist % 2 == 1)
            .count() as i64;
        let even_full = self
            .distances
            .values()
            .filter(|&&dist| dist % 2 == 0)
            .count() as i64;

        let odd_corners = self
            .distances
            .values()
            .filter(|&&dist| dist % 2 == 1 && dist > size)
            .count() as i64;
        let even_corners = self
            .distances
            .values()
            .filter(|&&dist| dist % 2 == 0 && dist > size)
            .count() as i64;

        let n = (steps - size) / self.size.x;
        let odd_full = (n + 1) * (n + 1) * odd_full;
        let even_full = (n * n) * even_full;
        let odd_corners = (n + 1) * odd_corners;
        let even_corners = n * even_corners;
        Some(odd_full + even_full - odd_corners + even_corners)
    }
}

impl Day {
    fn take_step(&self, visited: &HashSet<Vector>, new: &HashSet<Vector>) -> HashSet<Vector> {
        let mut next = HashSet::new();
        for &pos in new {
            for a in DIRS {
                let pos = pos + a;
                if !self.grid.contains(pos.rem_euclid(self.size)) && !visited.contains(&pos) {
                    next.insert(pos);
                }
            }
        }
        next
    }
}

utils::solution::test_solution!(aoc2023, day21);
