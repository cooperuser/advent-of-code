use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE_A: &str = include_str!("input_sample.txt");
pub const SAMPLE_B: &str = SAMPLE_A;
pub const ANSWER_A: i64 = 41;
pub const ANSWER_B: i64 = 6;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    map: HashSet<Spot>,
    size: Spot,
    start: Spot,
}

type Spot = (i64, i64);

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn rotate_right(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::East => Self::South,
            Self::West => Self::North,
        }
    }

    fn as_vector(&self) -> Spot {
        match self {
            Direction::North => (-1, 0),
            Direction::South => (1, 0),
            Direction::East => (0, 1),
            Direction::West => (0, -1),
        }
    }
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut map = HashSet::new();
        let mut start = (0, 0);
        for (y, line) in raw.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let spot = (y as i64, x as i64);
                if c == '#' {
                    map.insert(spot);
                } else if c == '^' {
                    start = spot;
                }
            }
        }

        Self {
            raw: raw.clone(),
            map,
            start,
            size: (raw.len() as i64, raw[0].len() as i64),
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        Some(self.count(None).unwrap().len() as i64)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut total = 0;
        for spot in self.count(None).unwrap() {
            if spot == self.start {
                continue;
            } else if self.count(Some(spot)).is_none() {
                total += 1;
            }
        }

        Some(total)
    }

    fn count(&self, extra: Option<Spot>) -> Option<HashSet<Spot>> {
        let mut facing = Direction::North;
        let mut seen: HashSet<Spot> = HashSet::new();
        let mut seen_facing: HashSet<(Spot, Direction)> = HashSet::new();
        let mut pos = self.start;
        loop {
            if seen_facing.contains(&(pos, facing)) {
                return None;
            }
            seen.insert(pos);
            seen_facing.insert((pos, facing));
            let dir = facing.as_vector();
            let next = (pos.0 + dir.0, pos.1 + dir.1);
            if next.0 < 0 || next.1 < 0 || next.0 >= self.size.0 || next.1 >= self.size.1 {
                return Some(seen);
            } else if self.map.contains(&next) || Some(next) == extra {
                facing = facing.rotate_right();
            } else {
                pos = next;
            }
        }
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
