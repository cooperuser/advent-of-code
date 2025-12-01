use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    rotations: Vec<i64>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 3,
            answer_b: 6,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut rotations = Vec::new();
        for line in &raw {
            let (l, r) = line.split_at(1);
            let value: i64 = r.parse().unwrap();
            rotations.push(value * if l == "R" { 1 } else { -1 });
        }
        Self { raw, rotations }
    }

    fn part_a(&self) -> Option<i64> {
        let mut dial = 50;
        let mut count = 0;
        for rot in &self.rotations {
            dial += rot;
            if dial.rem_euclid(100) == 0 {
                count += 1;
            }
        }
        Some(count)
    }

    fn part_b(&self) -> Option<i64> {
        let mut dial = 50;
        let mut count = 0;
        for rot in &self.rotations {
            for _ in 0..rot.abs() {
                dial += rot.signum();
                if dial.rem_euclid(100) == 0 {
                    count += 1;
                }
            }
        }
        Some(count)
    }
}

utils::solution::test_solution!(aoc2025, day01);
