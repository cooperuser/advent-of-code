use utils::{
    prelude::*,
    vector::{Vector, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    paper: VectorSet,
}

const ADJACENT: [Vector; 8] = [
    Vector::new(-1, -1),
    Vector::new(-1, 0),
    Vector::new(-1, 1),
    Vector::new(0, -1),
    Vector::new(0, 1),
    Vector::new(1, -1),
    Vector::new(1, 0),
    Vector::new(1, 1),
];

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
        Self { raw, paper }
    }

    fn part_a(&self) -> Option<i64> {
        let mut accessible = 0;

        for pos in self.paper.iter() {
            let neighbors = ADJACENT
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
            let mut next_paper = paper.clone();

            for pos in paper.iter() {
                let neighbors = ADJACENT
                    .iter()
                    .filter(|&&adj| paper.contains(pos + adj))
                    .count();

                if neighbors < 4 {
                    next_paper.remove(pos);
                    removed += 1;
                    finished = false;
                }
            }

            paper = next_paper;
        }

        Some(removed)
    }
}

utils::solution::test_solution!(aoc2025, day04);
