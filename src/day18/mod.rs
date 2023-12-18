use std::collections::{HashSet, VecDeque};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 62;
pub const ANSWER_B: i64 = 0;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    plans: Vec<Plan>,
}

type Pos = (i64, i64);
const DIRS: [Dir; 4] = [Dir::North, Dir::South, Dir::East, Dir::West];

#[derive(Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn add(&self, pos: Pos) -> Pos {
        match self {
            Dir::North => (pos.0 - 1, pos.1),
            Dir::South => (pos.0 + 1, pos.1),
            Dir::East => (pos.0, pos.1 + 1),
            Dir::West => (pos.0, pos.1 - 1),
        }
    }
}

#[derive(Debug)]
struct Plan {
    direction: Dir,
    count: i64,
    color: String,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut plans = Vec::new();
        for line in &raw {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() != 3 {
                continue;
            }
            plans.push(Plan {
                direction: match parts[0] {
                    "U" => Dir::North,
                    "D" => Dir::South,
                    "R" => Dir::East,
                    "L" => Dir::West,
                    _ => panic!("Unknown direction {}", parts[0]),
                },
                count: parts[1].parse().unwrap(),
                color: parts[2][1..parts[2].len() - 1].to_string()
            })
        }
        Self {
            raw: raw.clone(),
            plans,
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut min = (0, 0);
        let mut max = (0, 0);
        let mut pos = (0, 0);
        let mut map = HashSet::from([pos]);
        for plan in &self.plans {
            for _ in 0..plan.count {
                pos = plan.direction.add(pos);
                map.insert(pos);
                min.0 = min.0.min(pos.0);
                min.1 = min.1.min(pos.1);
                max.0 = max.0.max(pos.0);
                max.1 = max.1.max(pos.1);
            }
        }

        let mut queue = VecDeque::from([(1, 1)]);
        let mut inside = HashSet::new();

        while let Some(pos) = queue.pop_front() {
            if map.contains(&pos) || inside.contains(&pos) {
                continue;
            }

            inside.insert(pos);
            for dir in DIRS {
                queue.push_back(dir.add(pos));
            }
        }

        Some(inside.len() as i64 + map.len() as i64)
    }

    pub fn part_b(&self) -> Option<i64> {
        None
    }
}

fn debug(map: &HashSet<Pos>, inside: &HashSet<Pos>, size: Pos) {
    for row in 0..size.0 {
        for col in 0..size.1 {
            let pos = (row, col);
            let c = if inside.contains(&pos) {
                '#'
            } else if map.contains(&pos) {
                'O'
            } else {
                '.'
            };
            print!("{}", c);
        }
        println!();
    }
    println!();
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
