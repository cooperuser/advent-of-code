#![allow(dead_code)]

use std::collections::{HashSet, HashMap};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i64 = 13;
pub const SAMPLE_B: i64 = 30;

struct Card {
    winning: HashSet<usize>,
    actual: HashSet<usize>
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    cards: Vec<Card>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut cards = Vec::new();
        for line in &raw {
            let (_, right) = line.split_once(": ").unwrap();
            let (winning, actual) = right.split_once(" | ").unwrap();
            cards.push(Card {
                winning: winning.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect(),
                actual: actual.split_whitespace().map(|n| n.parse::<usize>().unwrap()).collect(),
            })
        }
        Self {
            raw: raw.clone(),
            cards,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut total = 0;
        for card in &self.cards {
            let count = card.winning
                .intersection(&card.actual)
                .collect::<HashSet<_>>()
                .len();
            if count != 0 {
                total += 2i64.pow(count as u32 - 1);
            }
        }
        Some(total)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut scaling: HashMap<usize, i64> = HashMap::from([(0, 1)]);
        for (index, card) in self.cards.iter().enumerate() {
            if !scaling.contains_key(&index) { scaling.insert(index, 1); }
            let count = scaling.get(&index).unwrap_or(&1).clone();
            let winning = card.winning
                .intersection(&card.actual)
                .collect::<HashSet<_>>()
                .len();
            if winning != 0 {
                for i in index+1..index+1+winning {
                    let s = scaling.get(&i).unwrap_or(&1);
                    scaling.insert(i, s + count);
                }
            }
        }
        Some(scaling.values().sum())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_a().unwrap_or(0), SAMPLE_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_b().unwrap_or(0), SAMPLE_B);
    }
}
