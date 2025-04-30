use std::collections::HashSet;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    cards: Vec<Card>,
}

struct Card {
    winning: HashSet<u8>,
    numbers: HashSet<u8>,
    points: usize,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 13,
            answer_b: 30,
        }
    }

    fn new(raw: Vec<String>) -> Self {
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
            let points = winning.intersection(&numbers).count();
            cards.push(Card {
                winning,
                numbers,
                points,
            });
        }

        Self {
            raw: raw.clone(),
            cards,
        }
    }

    fn part_a(&self) -> Option<i64> {
        Some(
            self.cards
                .iter()
                .map(|c| 2i64.pow(c.points as u32 - 1))
                .sum(),
        )
    }

    fn part_b(&self) -> Option<i64> {
        let mut copies = vec![1; self.cards.len()];
        for (i, card) in self.cards.iter().enumerate() {
            let count = copies[i];
            for p in 1..=card.points {
                copies[i + p] += count;
            }
        }
        Some(copies.into_iter().sum::<usize>() as i64)
    }
}

crate::solution::test_solution!();
