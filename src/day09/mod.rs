use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 88; // 13
pub const SAMPLE_B: i32 = 36;

const UP: (i32, i32) = (0, -1);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);
const RIGHT: (i32, i32) = (1, 0);

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    insts: Vec<(i32, i32)>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut insts = vec![];
        for line in &raw {
            let split: Vec<&str> = line.split_whitespace().collect();
            let dir = match split[0] {
                "U" => UP,
                "D" => DOWN,
                "L" => LEFT,
                "R" => RIGHT,
                _ => panic!()
            };
            for _ in 0..split[1].parse::<usize>().unwrap() {
                insts.push(dir);
            }
        }
        Self {
            raw: raw.clone(),
            insts,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut head = (0, 0);
        let mut tail = (0, 0);
        for dir in &self.insts {
            head.0 += dir.0;
            head.1 += dir.1;
            let diff = (head.0 - tail.0, head.1 - tail.1);
            if diff.0 == 2 || diff.0 == -2 {
                tail.1 = head.1;
                tail.0 += diff.0 / 2;
            } else if diff.1 == 2 || diff.1 == -2 {
                tail.0 = head.0;
                tail.1 += diff.1 / 2;
            }
            visited.insert(tail);
        }
        visited.len() as i32
    }

    pub fn part_b(&self) -> i32 {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut head = (0, 0);
        let mut tails = vec![];
        for _ in 0..9 { tails.push((0, 0)); }
        for dir in &self.insts {
            head.0 += dir.0;
            head.1 += dir.1;
            for i in 0..tails.len() {
                let head = if i == 0 { head }
                else { tails[i - 1] };
                let diff = (head.0 - tails[i].0, head.1 - tails[i].1);

                if diff.0.abs() >= 2 || diff.1.abs() >= 2 {
                    tails[i].0 += diff.0.signum();
                    tails[i].1 += diff.1.signum();
                }
            }
            visited.insert(*tails.last().unwrap());
        }
        visited.len() as i32
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
