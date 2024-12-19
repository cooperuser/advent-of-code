use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
    towels: Vec<Vec<Color>>,
    patterns: HashMap<Color, BinaryHeap<Pattern>>,
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
type Memo2 = HashMap<Vec<Color>, usize>;

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
            towels,
            patterns: sorted,
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
        let mut count = 0;
        let mut memo: Memo2 = HashMap::new();
        for towel in &self.towels {
            count += self.get_towel_possibilities(towel, &mut memo);
        }
        Some(count as i64)
    }
}

impl Day {
    fn is_towel_possible(&self, towel: &[Color], memo: &mut Memo) -> Option<Vec<Vec<Color>>> {
        if let Some(output) = memo.get(&towel.to_vec()) {
            return output.clone();
        }

        for Pattern { pattern: p } in self.patterns.get(&towel[0])? {
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

    fn get_towel_possibilities(&self, towel: &[Color], memo: &mut Memo2) -> usize {
        if let Some(output) = memo.get(&towel.to_vec()) {
            return *output;
        }

        let sorted = match self.patterns.get(&towel[0]) {
            Some(s) => s,
            None => return 0,
        };

        let mut possible = 0;
        for Pattern { pattern: p } in sorted {
            if p == towel {
                possible += 1;
            } else if p.len() > towel.len() {
                continue;
            } else if *p == towel[0..p.len()] {
                possible += self.get_towel_possibilities(&towel[p.len()..], memo)
            }
        }

        memo.insert(towel.to_vec(), possible);
        possible
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
