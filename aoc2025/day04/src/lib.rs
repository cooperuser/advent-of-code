use utils::{
    prelude::*,
    vector::{Vector, VectorSet},
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    paper: VectorSet,
    size: Vector,
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
        let size = Vector::new_usize(raw[0].len(), raw.len());
        let mut paper = VectorSet::new(size);

        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '@' {
                    paper.insert(Vector::new_usize(x, y));
                }
            }
        }

        Self { raw, paper, size }
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
        let mut removed = true;
        let mut num_removed = 0;

        while removed {
            removed = false;
            let mut next_paper = paper.clone();
            for y in 0..self.size.y {
                for x in 0..self.size.x {
                    if !paper.contains(Vector::new(x, y)) {
                        continue;
                    }

                    let mut neighbors = 0;
                    for i in -1..=1 {
                        for j in -1..=1 {
                            if i == 0 && j == 0 {
                                continue;
                            }

                            if paper.contains(Vector::new(x + i, y + j)) {
                                neighbors += 1;
                            }
                        }
                    }

                    if neighbors < 4 {
                        next_paper.remove(Vector::new(x, y));
                        num_removed += 1;
                        removed = true;
                    }
                }
            }
            paper = next_paper;
        }

        Some(num_removed)
    }
}

utils::solution::test_solution!(aoc2025, day04);
