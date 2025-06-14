use crate::prelude::*;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    grid: Vec<Vec<char>>,
    length: i64,
    width: i64,
}

type Point = (i64, i64);
const DIRS: &[Point] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 18,
            answer_b: 9,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        Self {
            raw: raw.clone(),
            grid: raw.iter().map(|line| line.chars().collect()).collect(),
            length: raw.len() as i64,
            width: raw[0].len() as i64,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let word = &['X', 'M', 'A', 'S'];
        let mut total = 0;

        for y in 0..self.length {
            for x in 0..self.width {
                'dir: for dir in DIRS {
                    let mut p = (y, x);
                    for letter in word {
                        if p.0 < 0
                            || p.1 < 0
                            || p.0 >= self.length
                            || p.1 >= self.width
                            || self.grid[p.0 as usize][p.1 as usize] != *letter
                        {
                            continue 'dir;
                        }
                        p = (p.0 + dir.0, p.1 + dir.1);
                    }

                    total += 1;
                }
            }
        }

        Some(total)
    }

    fn part_b(&self) -> Option<i64> {
        let word = &['M', 'A', 'S'];
        let mut total = 0;
        let mut spots: HashSet<Point> = HashSet::new();

        for y in 0..self.length {
            for x in 0..self.width {
                'dir: for dir in DIRS {
                    if dir.0 == 0 || dir.1 == 0 {
                        continue;
                    }

                    let mut p = (y, x);
                    for letter in word {
                        if p.0 < 0
                            || p.1 < 0
                            || p.0 >= self.length
                            || p.1 >= self.width
                            || self.grid[p.0 as usize][p.1 as usize] != *letter
                        {
                            continue 'dir;
                        }
                        p = (p.0 + dir.0, p.1 + dir.1);
                    }

                    let spot = (y + dir.0, x + dir.1);
                    if !spots.insert(spot) {
                        total += 1;
                    }
                }
            }
        }

        Some(total)
    }
}

crate::solution::test_solution!();
