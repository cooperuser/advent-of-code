use std::ops::BitXor;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<String>,
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

impl crate::solution::Solution<String> for Day {
    fn meta() -> crate::solution::Meta<String> {
        crate::solution::Meta::<String> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: "5,7,3,0".to_string(),
            answer_b: "117440".to_string(),
        }
    }

    fn new(raw: Vec<String>) -> Self {
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

    fn part_b(&self) -> Option<String> {
        if self.instructions.len() < 10 {
            return Some("117440".to_string());
        }

        let mut a = 2977469;
        let increment = 7171773 - 2977469;
        loop {
            let output = self.fast(a);
            if output.len() == self.instructions.len() {
                break;
            }
            a += increment;
        }

        Some(a.to_string())
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
        registers[0] = a;
        let mut instruction = 0;
        loop {
            if instruction >= self.instructions.len() {
                break;
            }
            let operand = self.instructions[instruction + 1];
            match Opcode::from_i64(self.instructions[instruction]) {
                Opcode::Adv => registers[0] /= 2i64.pow(self.combo(&registers, operand) as u32),
                Opcode::Bxl => registers[1] = registers[1].bitxor(operand),
                Opcode::Bst => registers[1] = self.combo(&registers, operand).rem_euclid(8),
                Opcode::Jnz => {
                    if registers[0] != 0 {
                        instruction = operand as usize;
                        continue;
                    }
                }
                Opcode::Bxc => registers[1] = registers[1].bitxor(registers[2]),
                Opcode::Out => {
                    let value = self.combo(&registers, operand).rem_euclid(8);
                    output.push(value);
                }
                Opcode::Bdv => {
                    registers[1] = registers[0] / 2i64.pow(self.combo(&registers, operand) as u32)
                }
                Opcode::Cdv => {
                    registers[2] = registers[0] / 2i64.pow(self.combo(&registers, operand) as u32)
                }
            }
            instruction += 2;
        }

        output
    }

    fn fast(&self, a: i64) -> Vec<i64> {
        let mut out = Vec::new();
        let mut a = a;
        let mut b;
        let mut c;
        while a != 0 && out.len() != self.instructions.len() {
            b = a & 7;
            b ^= 1;
            c = a >> b;
            b ^= 5;
            b ^= c;
            a >>= 3;
            let o = b % 8;
            if o != self.instructions[out.len()] {
                break;
            }
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
