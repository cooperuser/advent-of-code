use crate::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    registers: Vec<i64>,
    instructions: Vec<i64>,
}

enum Opcode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Solution<String, i64> for Day {
    fn meta() -> Meta<String, i64> {
        Meta::<String, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: "5,7,3,0".to_string(),
            answer_b: 117440,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let (registers, program) = raw.split_once(|line| line.is_empty()).unwrap();
        let registers = registers
            .iter()
            .map(|line| line.split_once(": ").unwrap().1.parse().unwrap())
            .collect();
        let program: Vec<_> = program[0].split_once(": ").unwrap().1.split(',').collect();
        let instructions = program.iter().map(|n| n.parse().unwrap()).collect();
        Self {
            raw: raw.clone(),
            registers,
            instructions,
        }
    }

    fn part_a(&self) -> Option<String> {
        let output = self.compute(self.registers[0]);
        Some(
            output
                .iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(","),
        )
    }

    fn part_b(&self) -> Option<i64> {
        let mut saved = Vec::new();
        for a in 1..1024 {
            let output = self.compute(a);
            if output[0] == self.instructions[0] {
                saved.push(a);
            }
        }

        for pos in 1..self.instructions.len() {
            let mut next = Vec::new();
            for s in saved {
                for bit in 0..8 {
                    let a = (bit << (7 + 3 * pos)) | s;
                    let output = self.compute(a);
                    if output.len() > pos && output[pos] == self.instructions[pos] {
                        next.push(a);
                    }
                }
            }

            saved = next;
        }

        saved.iter().min().copied()
    }
}

impl Day {
    const fn combo(&self, registers: &[i64], operand: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => registers[0],
            5 => registers[1],
            6 => registers[2],
            7 => panic!(),
            _ => panic!(),
        }
    }

    fn compute(&self, a: i64) -> Vec<i64> {
        let mut output: Vec<i64> = Vec::new();
        let mut registers = self.registers.clone();
        let mut instruction = 0;
        registers[0] = a;

        while instruction < self.instructions.len() {
            let opcode = self.instructions[instruction];
            let operand = self.instructions[instruction + 1];
            instruction += 2;

            match Opcode::from_i64(opcode) {
                Opcode::Adv => registers[0] >>= self.combo(&registers, operand),
                Opcode::Bxl => registers[1] ^= operand,
                Opcode::Bst => registers[1] = self.combo(&registers, operand) % 8,
                Opcode::Jnz if registers[0] != 0 => instruction = operand as usize,
                Opcode::Jnz => {}
                Opcode::Bxc => registers[1] ^= registers[2],
                Opcode::Out => output.push(self.combo(&registers, operand) % 8),
                Opcode::Bdv => registers[1] = registers[0] >> self.combo(&registers, operand),
                Opcode::Cdv => registers[2] = registers[0] >> self.combo(&registers, operand),
            }
        }

        output
    }

    #[allow(dead_code)]
    fn fast_compute(&self, a: i64) -> Vec<i64> {
        let mut out = Vec::new();
        let mut a = a;
        let mut b;
        let mut c;
        while a != 0 {
            b = a & 7;
            b ^= 1;
            c = a >> b;
            b ^= 5;
            b ^= c;
            a >>= 3;
            out.push(b % 8);
        }
        out
    }
}

impl Opcode {
    const fn from_i64(value: i64) -> Self {
        match value {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            _ => panic!(),
        }
    }
}

crate::solution::test_solution!();
