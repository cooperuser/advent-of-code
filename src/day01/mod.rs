#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i64 = 209;
pub const SAMPLE_B: i64 = 281;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i64 {
        let mut sum = 0;
        for line in &self.raw {
            let mut first = 0;
            let mut last = 0;
            for char in line.chars() {
                if !char.is_numeric() { continue; }
                if first == 0 {
                    first = char.to_digit(10).unwrap();
                }
                last = char.to_digit(10).unwrap();
            }
            sum += first * 10 + last;
        }
        sum as i64
    }

    pub fn part_b(&self) -> i64 {
        let mut sum = 0;
        for line in &self.raw {
            let mut nums: Vec<u32> = Vec::new();
            let mut chars: Vec<String> = Vec::new();
            for char in line.chars() {
                if char.is_numeric() {
                    nums.push(char.to_digit(10).unwrap());
                    continue;
                }
                chars.push(char.to_string());
                let len = chars.len() as i32;

                if chars[(len-4).max(0) as usize..].join("") == "zero" { nums.push(0) }
                else if chars[(len-3).max(0) as usize..].join("") == "one" { nums.push(1) }
                else if chars[(len-3).max(0) as usize..].join("") == "two" { nums.push(2) }
                else if chars[(len-5).max(0) as usize..].join("") == "three" { nums.push(3) }
                else if chars[(len-4).max(0) as usize..].join("") == "four" { nums.push(4) }
                else if chars[(len-4).max(0) as usize..].join("") == "five" { nums.push(5) }
                else if chars[(len-3).max(0) as usize..].join("") == "six" { nums.push(6) }
                else if chars[(len-5).max(0) as usize..].join("") == "seven" { nums.push(7) }
                else if chars[(len-5).max(0) as usize..].join("") == "eight" { nums.push(8) }
                else if chars[(len-4).max(0) as usize..].join("") == "nine" { nums.push(9) }
            }
            let first = nums.first().unwrap();
            let last = nums.last().unwrap();
            sum += first * 10 + last;
        }
        sum as i64
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
