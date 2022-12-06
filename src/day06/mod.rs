use std::collections::HashSet;

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 7;
pub const SAMPLE_B: i32 = 19;

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    stream: Vec<char>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        Self {
            raw: raw.clone(),
            stream: raw[0].chars().collect(),
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut queue: Vec<char> = vec![];
        for (i, &c) in self.stream.iter().enumerate() {
            queue.push(c);
            if queue.len() > 4 { queue.remove(0); }
            let mut set: HashSet<char> = HashSet::new();
            for &q in &queue { set.insert(q); }
            if set.len() == 4 { return i as i32 + 1 }
        }
        panic!("solution not found");
    }

    pub fn part_b(&self) -> i32 {
        let mut queue: Vec<char> = vec![];
        for (i, &c) in self.stream.iter().enumerate() {
            queue.push(c);
            if queue.len() > 14 { queue.remove(0); }
            let mut set: HashSet<char> = HashSet::new();
            for &q in &queue { set.insert(q); }
            if set.len() == 14 { return i as i32 + 1 }
        }
        panic!("solution not found");
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
