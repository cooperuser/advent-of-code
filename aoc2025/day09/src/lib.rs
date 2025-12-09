use utils::{prelude::*, vector::Vector};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    tiles: Vec<Vector>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 50,
            answer_b: 24,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut points = Vec::new();
        for line in &raw {
            let (x, y) = line.split_once(',').unwrap();
            let x = x.parse().unwrap();
            let y = y.parse().unwrap();
            points.push(Vector::new(x, y));
        }
        Self { raw, tiles: points }
    }

    fn part_a(&self) -> Option<i64> {
        let mut max = 0;

        for (i, &a) in self.tiles.iter().enumerate() {
            for &b in self.tiles.iter().skip(i + 1) {
                let diff = (b - a).abs() + Vector::new(1, 1);
                max = max.max(diff.area());
            }
        }

        Some(max)
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

utils::solution::test_solution!(aoc2025, day09);
