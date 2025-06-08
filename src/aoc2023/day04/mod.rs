use std::{collections::HashSet, rc::Rc};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    cards: Vec<u32>,
}

impl crate::solution::Solution<u32, u32> for Day {
    fn meta() -> crate::solution::Meta<u32, u32> {
        crate::solution::Meta::<u32, u32> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 13,
            answer_b: 30,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut cards = Vec::new();
        for line in raw.iter() {
            let parts = line.split_once(": ").unwrap().1;
            let (winning, numbers) = parts.split_once(" | ").unwrap();
            let winning: HashSet<u8> = winning
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            let numbers: HashSet<u8> = numbers
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            cards.push(winning.intersection(&numbers).count() as u32);
        }

        Self {
            raw: raw.clone(),
            cards,
        }
    }

    fn part_a(&self) -> Option<u32> {
        Some(self.cards.iter().map(|&c| 2u32.pow(c - 1)).sum())
    }

    fn part_b(&self) -> Option<u32> {
        let mut copies = vec![1; self.cards.len()];
        for (i, &card) in self.cards.iter().enumerate() {
            let count = copies[i];
            for p in 1..=card as usize {
                copies[i + p] += count;
            }
        }
        Some(copies.into_iter().sum())
    }
}

crate::solution::test_solution!();
