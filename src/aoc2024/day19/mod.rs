use crate::prelude::*;
use std::collections::HashMap;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    towels: Vec<Vec<Color>>,
    patterns: HashMap<Color, Vec<Pattern>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Clone, Eq, PartialEq)]
struct Pattern(Vec<Color>);

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 6,
            answer_b: 16,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let (patterns, towels) = raw.split_once(|line| line.is_empty()).unwrap();
        let patterns: Vec<Pattern> = patterns[0]
            .split(", ")
            .map(|p| p.chars().map(|c| c.into()).collect())
            .map(Pattern)
            .collect();
        let mut sorted: HashMap<Color, Vec<Pattern>> = HashMap::new();
        for pattern in &patterns {
            sorted
                .entry(pattern.0[0])
                .or_default()
                .push(pattern.clone());
        }
        let towels = towels
            .iter()
            .map(|t| t.chars().map(|c| c.into()).collect())
            .collect();

        Self {
            raw: raw.clone(),
            towels,
            patterns: sorted,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut count = 0;
        let mut memo = HashMap::new();
        for towel in &self.towels {
            if self.is_towel_possible(towel, &mut memo) {
                count += 1;
            }
        }
        Some(count)
    }

    fn part_b(&self) -> Option<i64> {
        let mut count = 0;
        let mut memo = HashMap::new();
        for towel in &self.towels {
            count += self.get_possibilities(towel, &mut memo);
        }
        Some(count as i64)
    }
}

impl Day {
    fn is_towel_possible(&self, towel: &[Color], memo: &mut HashMap<Vec<Color>, bool>) -> bool {
        if let Some(&output) = memo.get(&towel.to_vec()) {
            return output;
        }

        let sorted = match self.patterns.get(&towel[0]) {
            Some(s) => s,
            None => return false,
        };

        for Pattern(p) in sorted {
            if p.len() > towel.len() {
                continue;
            }
            if p == towel
                || *p == towel[0..p.len()] && self.is_towel_possible(&towel[p.len()..], memo)
            {
                return true;
            }
        }

        memo.insert(towel.to_vec(), false);
        false
    }

    fn get_possibilities(&self, towel: &[Color], memo: &mut HashMap<Vec<Color>, usize>) -> usize {
        if let Some(&output) = memo.get(&towel.to_vec()) {
            return output;
        }

        let sorted = match self.patterns.get(&towel[0]) {
            Some(s) => s,
            None => return 0,
        };

        let mut possible = 0;
        for Pattern(p) in sorted {
            if p == towel {
                possible += 1;
            } else if p.len() > towel.len() {
                continue;
            } else if *p == towel[0..p.len()] {
                possible += self.get_possibilities(&towel[p.len()..], memo)
            }
        }

        memo.insert(towel.to_vec(), possible);
        possible
    }
}

impl From<char> for Color {
    fn from(value: char) -> Self {
        match value {
            'w' => Self::White,
            'u' => Self::Blue,
            'b' => Self::Black,
            'r' => Self::Red,
            'g' => Self::Green,
            _ => panic!("Unrecognized color {value}"),
        }
    }
}

impl Ord for Pattern {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.len().cmp(&other.0.len()).reverse()
    }
}

impl PartialOrd for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

crate::solution::test_solution!();
