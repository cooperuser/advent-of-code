use std::{collections::HashMap, rc::Rc};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    stones: HashMap<i64, i64>,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 55312,
            answer_b: 65601038650482,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut stones = HashMap::new();
        for number in raw[0].split_whitespace() {
            stones.entry(number.parse().unwrap()).or_insert(1);
        }
        Self {
            raw: raw.clone(),
            stones,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut stones = self.stones.clone();
        for _ in 0..25 {
            stones = Self::blink(&stones);
        }
        Some(stones.values().sum())
    }

    fn part_b(&self) -> Option<i64> {
        let mut stones = self.stones.clone();
        for _ in 0..75 {
            stones = Self::blink(&stones);
        }
        Some(stones.values().sum())
    }
}

impl Day {
    fn blink(stones: &HashMap<i64, i64>) -> HashMap<i64, i64> {
        let mut after = HashMap::new();
        for (&stone, &count) in stones.iter() {
            if stone == 0 {
                *after.entry(1).or_insert(0) += count;
            } else if (stone.ilog10() + 1) % 2 == 0 {
                let digits = stone.ilog10() + 1;
                let power = 10i64.pow(digits / 2);
                let right = stone % power;
                let left = (stone - right) / power;
                *after.entry(left).or_insert(0) += count;
                *after.entry(right).or_insert(0) += count;
            } else {
                *after.entry(stone * 2024).or_insert(0) += count;
            }
        }
        after
    }
}

crate::solution::test_solution!();
