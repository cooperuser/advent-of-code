pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: &str = "2=-1=0";
pub const SAMPLE_B: i64 = 0;

fn balloon_to_num(balloon: &String) -> i64 {
    let len = balloon.len();
    let mut num = 0;
    for (i, c) in balloon.chars().enumerate() {
        let value = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => panic!()
        };
        let index = len - i - 1;
        num += value * 5i64.pow(index as u32);
    }
    num
}

fn num_to_balloon(num: i64) -> String {
    let mut chars: Vec<&str> = vec![];
    let mut num = num;
    while num > 0 {
        let index = (num % 5) as usize;
        let d = [0, 1, 2, -2, -1][index];
        chars.push(["0", "1", "2", "=", "-"][index]);
        num -= d;
        num /= 5;
    }
    chars.reverse();
    chars.join("")
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    balloons: Vec<String>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            balloons: raw.iter().map(|line| line.to_string()).collect(),
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> String {
        let sum = self.balloons.iter().map(balloon_to_num).sum();
        num_to_balloon(sum)
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
