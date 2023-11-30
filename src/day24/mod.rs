#![allow(dead_code)]

use std::collections::{VecDeque, HashSet, HashMap};

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 18;
pub const SAMPLE_B: i32 = 0;

type Pos = (i32, i32);
const DIRS: [Pos; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Clone, Default)]
struct Blizzards {
    up: HashSet<Pos>,
    down: HashSet<Pos>,
    left: HashSet<Pos>,
    right: HashSet<Pos>,
    bounds: Pos,
}

impl Blizzards {
    fn has_blizzard_at(&self, pos: &Pos, round: i32) -> bool {
        let mut up_pos = ((pos.0 - round) % self.bounds.0, pos.1);
        if up_pos.0 < 0 { up_pos.0 += self.bounds.0 }
        let down_pos = ((pos.0 + round) % self.bounds.0, pos.1);
        let mut left_pos = (pos.0, (pos.1 - round) % self.bounds.1);
        if left_pos.1 < 0 { left_pos.1 += self.bounds.1 }
        let right_pos = (pos.0, (pos.1 + round) % self.bounds.1);
        self.up.contains(&up_pos)
        || self.down.contains(&down_pos)
        || self.left.contains(&left_pos)
        || self.right.contains(&right_pos)
    }

    fn draw(&self, round: i32, pos: &Pos) {
        for y in 0..self.bounds.0 {
            for x in 0..self.bounds.1 {
                let spot = (y, x);
                if *pos == spot {
                    print!("@");
                } else if self.has_blizzard_at(&spot, round) {
                    print!("b");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn in_bounds(&self, pos: &Pos) -> bool {
        pos.0 >= 0 && pos.1 >= 0
        && pos.0 < self.bounds.0
        && pos.1 < self.bounds.1
    }
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    blizzards: Blizzards
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut up = HashSet::new();
        let mut down = HashSet::new();
        let mut left = HashSet::new();
        let mut right = HashSet::new();
        let bounds = (raw.len() as i32 - 2, raw[0].len() as i32 - 2);
        for (y, line) in raw.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let (y, x) = (y as i32 - 1, x as i32 - 1);
                match ch {
                    '#' => false,
                    '.' => false,
                    '^' => up.insert((y, x)),
                    'v' => down.insert((y, x)),
                    '<' => left.insert((y, x)),
                    '>' => right.insert((y, x)),
                    _ => panic!()
                };
            }
        }
        Self {
            raw: raw.clone(),
            blizzards: Blizzards {
                up,
                down,
                left,
                right,
                bounds
            },
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut queue: VecDeque<(Pos, i32)> = VecDeque::new();
        let bounds = self.blizzards.bounds;
        let start = (-1, 0);
        let end = (bounds.0, bounds.1 - 1);
        queue.push_back((start, 0));
        // let seen: HashMap<Pos, i32> = HashMap::new();
        // let mut last_round = 0;
        while let Some((pos, round)) = queue.pop_front() {
            // if round != last_round {
            //     println!("round {}: {:?}", round, pos);
            //     last_round = round;
            // }
            // println!("\n{}", round);
            // self.blizzards.draw(round, &pos);
            for dir in DIRS {
                let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                if new_pos == end { return round - 1 }
                if !(new_pos == start || self.blizzards.in_bounds(&new_pos)) { continue }
                // if !self.blizzards.in_bounds(&new_pos) { continue }
                if self.blizzards.has_blizzard_at(&new_pos, round + 1) { continue }
                queue.push_back((new_pos, round + 1));
            }
            if !self.blizzards.has_blizzard_at(&pos, round + 1) {
                queue.push_back((pos, round + 1));
            }
        }
        0
    }

    pub fn part_b(&self) -> i32 {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_a(), SAMPLE_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_b(), SAMPLE_B);
    }
}
