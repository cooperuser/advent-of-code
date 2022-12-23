#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 3068;
pub const SAMPLE_B: i64 = 1514285714288;

#[derive(Debug)]
enum Dir {
    Left,
    Right
}

impl Dir {
    fn parse(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            _ => panic!("Unable to parse char {c}")
        }
    }
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    dirs: Vec<Dir>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            dirs: raw[0].chars().map(Dir::parse).collect(),
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        println!("{:?}", self.dirs);
        0
    }

    pub fn part_b(&self) -> i64 {
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
