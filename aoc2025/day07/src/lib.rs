use std::collections::HashSet;

use utils::{
    prelude::*,
    vector::{Vector, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    map: VectorSet,
    size: Vector,
    start: i64,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 21,
            answer_b: 0,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let map = VectorSet::from_grid(&raw, '^');
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let start = raw[0].chars().position(|c| c == 'S').unwrap() as i64;
        Self {
            raw,
            map,
            size,
            start,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut beams = HashSet::new();
        let mut count = 0;
        beams.insert(self.start);
        for y in 0..self.size.y {
            let mut next = HashSet::new();
            for &beam in &beams {
                if self.map.contains(Vector::new(beam, y as i64)) {
                    next.insert(beam - 1);
                    next.insert(beam + 1);
                    count += 1;
                } else {
                    next.insert(beam);
                }
            }
            beams = next;
        }
        Some(count)
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

utils::solution::test_solution!(aoc2025, day08);
