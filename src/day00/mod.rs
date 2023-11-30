pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i64 = 0;
pub const SAMPLE_B: i64 = 0;

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
