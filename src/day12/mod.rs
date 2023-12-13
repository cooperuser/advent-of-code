#![allow(dead_code)]

use std::collections::HashMap;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 21;
pub const ANSWER_B: i64 = 525152;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    groups: Vec<Group>
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            _ => panic!("bad code")
        }
    }
}

#[derive(Debug)]
struct Group {
    springs: Vec<Spring>,
    numbers: Vec<usize>,
}

impl Group {
    fn count(&self, map: &mut HashMap<(usize, usize), i64>, s: usize, n: usize) -> i64 {
        if let Some(cached) = map.get(&(s, n)) {
            return *cached;
        }

        if n == self.numbers.len() {
            return if self.springs[s..].contains(&Spring::Damaged) {
                map.insert((s, n), 0);
                0
            } else {
                map.insert((s, n), 1);
                1
            }
        }

        let number = self.numbers[n];
        if self.springs.len() - s < number + 1 {
            map.insert((s, n), 0);
            return 0
        }

        let mut possible = 0;
        if self.springs[s] != Spring::Damaged {
            possible += self.count(map, s + 1, n);
        }
        if !self.springs[s..s + number].contains(&Spring::Operational) && self.springs[s + number] != Spring::Damaged {
            possible += self.count(map, s + number + 1, n + 1);
        }
        map.insert((s, n), possible);
        possible
    }
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut groups = Vec::new();
        for line in &raw {
            let (left, right) = line.split_once(' ').unwrap();
            groups.push(Group {
                springs: left.chars().map(|c| c.into()).collect(),
                numbers: right.split(',').map(|n| n.parse().unwrap()).collect()
            })
        }
        Self {
            raw: raw.clone(),
            groups,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut sum = 0;

        for group in &self.groups {
            let mut springs = group.springs.clone();
            springs.push(Spring::Operational);

            let group = Group {
                springs,
                numbers: group.numbers.clone()
            };

            sum += group.count(&mut HashMap::new(), 0, 0);
        }

        Some(sum)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut sum = 0;

        for group in &self.groups {
            let mut springs = Vec::new();
            for i in 0..5 {
                for spring in &group.springs {
                    springs.push(*spring);
                }
                if i != 4 {
                    springs.push(Spring::Unknown);
                }
            }
            springs.push(Spring::Operational);

            let group = Group {
                springs,
                numbers: group.numbers.repeat(5),
            };

            sum += group.count(&mut HashMap::new(), 0, 0);
        }

        Some(sum)
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
