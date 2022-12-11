#![allow(dead_code)]

pub const INPUT: &str = include_str!("input.txt");
pub const SAMPLE: &str = include_str!("input_sample.txt");
pub const SAMPLE_A: i32 = 13140;
pub const SAMPLE_B: i32 = 0;

enum Instruction {
    Noop,
    Addx(i32)
}

#[derive(Default)]
pub struct Solution {
    #[allow(dead_code)]
    raw: Vec<String>,
    insts: Vec<Instruction>
}

impl Solution {
    pub fn new(raw: Vec<String>) -> Self {
        let mut insts = vec![];
        for line in &raw {
            let split: Vec<&str> = line.split_whitespace().collect();
            match split[0] {
                "noop" => insts.push(Instruction::Noop),
                "addx" => insts.push(Instruction::Addx(split[1].parse().unwrap())),
                _ => panic!()
            }
        }
        Self {
            raw: raw.clone(),
            insts,
            ..Default::default()
        }
    }

    pub fn part_a(&self) -> i32 {
        let mut checks = vec![];
        let mut cycles = vec![];
        let mut register = 1;
        for inst in &self.insts {
            match inst {
                Instruction::Noop => cycles.push(0),
                Instruction::Addx(value) => {
                    cycles.push(0);
                    cycles.push(*value)
                },
            }
        }
        for i in 0..cycles.len() {
            if i % 40 == 19 {
                checks.push(register * (i as i32 + 1));
            }
            register += cycles[i];
        }
        checks.iter().sum()
    }

    pub fn part_b(&self) -> i32 {
        let mut cycles = vec![];
        let mut register = 1;
        for inst in &self.insts {
            match inst {
                Instruction::Noop => cycles.push(0),
                Instruction::Addx(value) => {
                    cycles.push(0);
                    cycles.push(*value)
                },
            }
        }
        for i in 0..cycles.len() {
            let pixel = (i % 40) as i32 + 1;
            register += cycles[i];
            if (register - pixel).abs() <= 1 {
                print!("#");
            } else {
                print!(".");
            }
            if pixel == 40 {
                println!("");
            }
        }
        println!("");
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
