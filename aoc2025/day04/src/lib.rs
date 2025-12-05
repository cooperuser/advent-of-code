use utils::{
    prelude::*,
    vector::{KINGS, Vector, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    paper: VectorSet,
    size: Vector,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 13,
            answer_b: 43,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let paper = VectorSet::from_grid(&raw, '@');
        let size = Vector::new_usize(raw[0].len(), raw.len());
        Self { raw, paper, size }
    }

    fn part_a(&self) -> Option<i64> {
        let mut accessible = 0;

        for pos in self.paper.iter() {
            let neighbors = KINGS
                .iter()
                .filter(|&&adj| self.paper.contains(pos + adj))
                .count();

            if neighbors < 4 {
                accessible += 1;
            }
        }

        Some(accessible)
    }

    fn part_b(&self) -> Option<i64> {
        let mut paper = self.paper.clone();
        let mut finished = false;
        let mut removed = 0;

        while !finished {
            finished = true;

            for pos in self.size.iter() {
                if !paper.contains(pos) {
                    continue;
                }

                let neighbors = KINGS
                    .iter()
                    .filter(|&&adj| paper.contains(pos + adj))
                    .count();

                if neighbors < 4 {
                    paper.remove(pos);
                    finished = false;
                    removed += 1;
                }
            }
        }

        Some(removed)
    }
}

utils::solution::test_solution!(aoc2025, day04);
