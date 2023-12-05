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
        }
    }

    pub fn part_a(&self) -> Option<i64> {
        let mut sum = 0;
        for line in &self.raw {
            let mut first = 0;
            let mut last = 0;
            for char in line.chars() {
                if !char.is_numeric() { continue; }
                if first == 0 { first = char.to_digit(10).unwrap(); }
                last = char.to_digit(10).unwrap();
            }
            sum += first * 10 + last;
        }
        Some(sum as i64)
    }

    pub fn part_b(&self) -> Option<i64> {
        let mut sum = 0;
        for line in &self.raw {
            let mut nums: Vec<u32> = Vec::new();
            let mut three: Vec<String> = Vec::new();
            let mut four: Vec<String> = Vec::new();
            let mut five: Vec<String> = Vec::new();

            for char in line.chars() {
                if char.is_numeric() {
                    nums.push(char.to_digit(10).unwrap());
                    continue;
                }
                three.push(char.to_string());
                four.push(char.to_string());
                five.push(char.to_string());

                if three.len() > 3 { three.remove(0); }
                if four.len() > 4 { four.remove(0); }
                if five.len() > 5 { five.remove(0); }

                let three = three.join("");
                let four = four.join("");
                let five = five.join("");

                if three == "one" { nums.push(1); }
                else if three == "two" { nums.push(2); }
                else if three == "six" { nums.push(6); }
                if four == "zero" { nums.push(0); }
                else if four == "four" { nums.push(4); }
                else if four == "five" { nums.push(5); }
                else if four == "nine" { nums.push(9); }
                if five == "three" { nums.push(3); }
                else if five == "seven" { nums.push(7); }
                else if five == "eight" { nums.push(8); }
            }
            let first = nums.first().unwrap();
            let last = nums.last().unwrap();
            sum += first * 10 + last;
        }
        Some(sum as i64)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_a() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_a().unwrap_or(0), SAMPLE_A);
    }

    #[test]
    fn part_b() {
        let solution = Solution::new(crate::split(SAMPLE));
        assert_eq!(solution.part_b().unwrap_or(0), SAMPLE_B);
    }
}
