use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    patterns: Vec<Pattern>,
    towels: Vec<Vec<Color>>,
    sorted: HashMap<Color, BinaryHeap<Pattern>>,
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

#[derive(Clone, Eq, PartialEq)]
struct Pattern {
    pattern: Vec<Color>,
}

type Memo = HashMap<Vec<Color>, Option<Vec<Vec<Color>>>>;

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 6,
            answer_b: 16,
        }
    }

    fn new(raw: Vec<String>) -> Self {
        let (patterns, towels) = raw.split_once(|line| line.is_empty()).unwrap();
        let patterns: Vec<Pattern> = patterns[0]
            .split(", ")
            .map(|p| p.chars().map(|c| c.to_string().parse().unwrap()).collect())
            .map(|p| Pattern { pattern: p })
            .collect();
        let mut sorted: HashMap<Color, BinaryHeap<Pattern>> = HashMap::new();
        for pattern in &patterns {
            sorted
                .entry(pattern.pattern[0])
                .or_default()
                .push(pattern.clone());
        }
        let towels = towels
            .iter()
            .map(|t| t.chars().map(|c| c.to_string().parse().unwrap()).collect())
            .collect();
        Self {
            raw: raw.clone(),
            patterns,
            towels,
            sorted,
        }
    }

    fn part_a(&self) -> Option<i64> {
        let mut count = 0;
        let mut memo: Memo = HashMap::new();
        for towel in &self.towels {
            if self.is_towel_possible(towel, &mut memo).is_some() {
                count += 1;
            }
        }
        Some(count)
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

impl Day {
    fn is_towel_possible(&self, towel: &[Color], memo: &mut Memo) -> Option<Vec<Vec<Color>>> {
        if let Some(output) = memo.get(&towel.to_vec()) {
            return output.clone();
        }

        for Pattern { pattern: p } in self.sorted.get(&towel[0])? {
            if p == towel {
                return Some(vec![p.clone()]);
            } else if p.len() > towel.len() {
                continue;
            } else if *p == towel[0..p.len()] {
                match self.is_towel_possible(&towel[p.len()..], memo) {
                    Some(mut output) => {
                        output.push(p.clone());
                        memo.insert(towel.to_vec(), Some(output.clone()));
                        return Some(output);
                    }
                    None => continue,
                }
            }
        }

        memo.insert(towel.to_vec(), None);
        None
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            'w' => Ok(Self::White),
            'u' => Ok(Self::Blue),
            'b' => Ok(Self::Black),
            'r' => Ok(Self::Red),
            'g' => Ok(Self::Green),
            _ => Err(()),
        }
    }
}

impl Ord for Pattern {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.pattern.len().cmp(&other.pattern.len()).reverse()
    }
}

impl PartialOrd for Pattern {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

crate::solution::test_solution!();
