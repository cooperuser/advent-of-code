#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: &str = "CMZ";
pub const SAMPLE_B: &str = "MCD";

#[derive(Default)]
pub struct Solution {
    raw: Vec<String>,
    stacks: Vec<Vec<char>>,
    commands: Vec<(usize, usize, i32)>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut stacks = vec![];
        let split = raw.iter().enumerate().find(|&r| r.1.to_string() == "".to_string()).unwrap().0;
        let num_stacks = raw[split - 1].split_whitespace().count();
        for _ in 0..num_stacks { stacks.push(vec![]); }

        for i in (0..split - 1).rev() {
            let line: Vec<char> = raw[i].chars().collect();
            for stack in 0..num_stacks {
                let c = line.get(4 * stack + 1);
                match c {
                    None => (),
                    Some(' ') => (),
                    Some(c) => stacks[stack].push(*c),
                }
            }
        }

        let mut commands = vec![];

        for i in split + 1..raw.len() {
            let line = raw[i].clone();
            let chars: Vec<&str> = line.split_whitespace().collect();
            let count: i32 = chars[1].to_string().parse().unwrap();
            let from: usize = chars[3].to_string().parse().unwrap();
            let to: usize = chars[5].to_string().parse().unwrap();
            commands.push((from - 1, to - 1, count));
        }

        Self {
            raw: raw.clone(),
            stacks,
            commands,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> String {
        let mut stacks = self.stacks.clone();
        for (from, to, count) in &self.commands {
            for _ in 0..*count {
                let c = stacks[*from].pop();
                stacks[*to].push(c.unwrap());
            }
        }

        let mut tops = "".to_string();
        for stack in stacks.clone() {
            tops = format!("{tops}{}", stack.last().unwrap());
        }
        tops
    }

    pub fn part_b(&self) -> String {
        let mut stacks = self.stacks.clone();
        for (from, to, count) in &self.commands {
            let mut pile = vec![];
            for _ in 0..*count {
                let c = stacks[*from].pop();
                pile.push(c.unwrap());
            }
            pile.reverse();
            stacks[*to].append(&mut pile);
        }

        let mut tops = "".to_string();
        for stack in stacks.clone() {
            tops = format!("{tops}{}", stack.last().unwrap());
        }
        tops
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
