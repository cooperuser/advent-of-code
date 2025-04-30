use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    hands: Vec<Hand>,
}

#[derive(PartialEq, Clone)]
struct Hand {
    name: Name,
    cards: String,
    bid: i64,
}

#[derive(PartialEq, Clone)]
enum Name {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl PartialOrd for Name {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some((self.clone() as u8).cmp(&(other.clone() as u8)))
    }
}

fn card_value(c: char, j: i64) -> i64 {
    if let Some(num) = c.to_digit(10) {
        return num as i64;
    }
    match c {
        'T' => 10,
        'J' => j,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => unreachable!(),
    }
}

fn sort(a: &Hand, b: &Hand, j: i64) -> Ordering {
    if a.name == b.name {
        for (c, d) in a.cards.chars().zip(b.cards.chars()) {
            let ord = card_value(c, j).cmp(&card_value(d, j));
            if ord != Ordering::Equal {
                return ord;
            }
        }
        Ordering::Equal
    } else {
        a.name.partial_cmp(&b.name).unwrap()
    }
}

fn substitute_jokers(cards: &str) -> Name {
    let chars = "23456789TQKA";
    let mut name = Name::HighCard;
    for p in chars.chars().map(|c| cards.replace('J', &c.to_string())) {
        let n = Name::new(&p);
        if n > name {
            name = n;
        }
    }
    name
}

impl Name {
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
            2 => match nums.contains(&1) {
                true => Self::FourOfAKind,
                false => Self::FullHouse,
            },
            3 => match nums.contains(&3) {
                true => Self::ThreeOfAKind,
                false => Self::TwoPair,
            },
            4 => Self::OnePair,
            _ => Self::HighCard,
        }
    }
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 6440,
            answer_b: 5905,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            hands: raw.iter().map(|h| h.parse().unwrap()).collect(),
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut hands = self.hands.clone();
        hands.sort_by(|a, b| sort(a, b, 11));

        let mut total = 0;
        for (rank, hand) in hands.iter().enumerate() {
            total += (rank as i64 + 1) * hand.bid;
        }

        Some(total)
    }

    fn part_b(&self) -> Option<i64> {
        let mut hands: Vec<_> = self
            .hands
            .iter()
            .map(|hand| Hand {
                name: substitute_jokers(&hand.cards),
                cards: hand.cards.clone(),
                bid: hand.bid,
            })
            .collect();
        hands.sort_by(|a, b| sort(a, b, 1));

        let mut total = 0;
        for (rank, hand) in hands.iter().enumerate() {
            total += (rank as i64 + 1) * hand.bid;
        }

        Some(total)
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let bid: i64 = bid.parse().unwrap();
        Ok(Hand {
            name: Name::new(cards),
            cards: cards.to_string(),
            bid,
        })
    }
}

crate::solution::test_solution!();
