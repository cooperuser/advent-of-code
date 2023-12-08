#![allow(dead_code)]

use std::{collections::{HashMap, HashSet}, cmp::Ordering};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 6440;
pub const ANSWER_B: i64 = 5905;

#[derive(PartialEq, Debug, Clone)]
struct Hand {
    r#type: Type,
    cards: String,
}

#[derive(PartialEq, Debug, Clone)]
enum Type {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.clone() as u8).cmp(&(other.clone() as u8)))
    }
}

fn card_value(c: char, j: i64) -> i64 {
    match c.to_digit(10) {
        Some(num) => num as i64,
        None => match c {
            'T' => 10,
            'J' => j,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => panic!()
        },
    }
}

fn subs_jokers(cards: &str) -> Type {
    let chars = "23456789TQKA";
    let mut r#type = Type::HighCard;
    for p in chars.chars().map(|c| cards.replace('J', &c.to_string())) {
        let t = Type::new(&p);
        if t > r#type { r#type = t; }
    }
    r#type
}

fn sort_func(a: &(Type, String, i64), b: &(Type, String, i64), j: i64) -> Ordering {
    if a.0 == b.0 {
        for (c, d) in a.1.chars().zip(b.1.chars()) {
            let ord = card_value(c, j).cmp(&card_value(d, j));
            if ord != Ordering::Equal { return ord }
        }
        Ordering::Equal
    } else {
        a.0.partial_cmp(&b.0).unwrap()
    }
}

impl Type {
    fn new(cards: &str) -> Self {
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in cards.chars() {
            match map.get(&c) {
                Some(count) => map.insert(c, count + 1),
                None => map.insert(c, 1),
            };
        }
        let nums: HashSet<&usize> = map.values().collect();
        match map.len() {
            1 => Self::FiveOfAKind,
            2 => {
                if nums.contains(&1) {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            },
            3 => {
                if nums.contains(&3) {
                    Self::ThreeOfAKind
                } else {
                    Self::TwoPair
                }
            },
            4 => {
                Self::OnePair
            }
            _ => Self::HighCard
        }
    }
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    hands: Vec<(Type, String, i64)>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut rounds = Vec::new();
        for line in &raw {
            let (cards, bid) = line.split_once(' ').unwrap();
            let bid: i64 = bid.parse().unwrap();
            rounds.push((Type::new(cards), cards.to_string(), bid));
        }
        Self {
            raw: raw.clone(),
            hands: rounds
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut hands = self.hands.clone();
        hands.sort_by(|a, b| { sort_func(a, b, 11) });

        let mut total = 0;
        for (rank, hand) in hands.iter().enumerate() {
            total += (rank as i64 + 1) * hand.2;
        }

        Some(total)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut hands: Vec<_> = self.hands.iter().map(|hand| {
            (subs_jokers(&hand.1), hand.1.clone(), hand.2)
        }).collect();
        hands.sort_by(|a, b| { sort_func(a, b, 1) });

        let mut total = 0;
        for (rank, hand) in hands.iter().enumerate() {
            total += (rank as i64 + 1) * hand.2;
        }

        Some(total)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE_A));
        assert_eq!(solution.part_a().unwrap_or(0), ANSWER_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE_B));
        assert_eq!(solution.part_b().unwrap_or(0), ANSWER_B);
    }
}
