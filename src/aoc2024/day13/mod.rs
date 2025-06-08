use std::rc::Rc;

use crate::vector::Vector;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    machines: Vec<Machine>,
}

struct Machine {
    a: Vector,
    b: Vector,
    prize: Vector,
}

impl crate::solution::Solution<i64, i64> for Day {
    fn meta() -> crate::solution::Meta<i64, i64> {
        crate::solution::Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 480,
            answer_b: 875318608908,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut machines = Vec::new();
        for m in raw.split(|line| line.is_empty()) {
            let a = m[0].split_once(": ").unwrap().1.split_once(", ").unwrap();
            let b = m[1].split_once(": ").unwrap().1.split_once(", ").unwrap();
            let prize = m[2].split_once(": ").unwrap().1.split_once(", ").unwrap();
            machines.push(Machine {
                a: Vector::new(
                    a.0.split_once('+').unwrap().1.parse().unwrap(),
                    a.1.split_once('+').unwrap().1.parse().unwrap(),
                ),
                b: Vector::new(
                    b.0.split_once('+').unwrap().1.parse().unwrap(),
                    b.1.split_once('+').unwrap().1.parse().unwrap(),
                ),
                prize: Vector::new(
                    prize.0.split_once('=').unwrap().1.parse().unwrap(),
                    prize.1.split_once('=').unwrap().1.parse().unwrap(),
                ),
            });
        }

        Self {
            raw: raw.clone(),
            machines,
        }
    }

    fn part_a(&self) -> Option<i64> {
        Some(
            self.machines
                .iter()
                .flat_map(|m| Self::solve_machine(m, 0))
                .sum(),
        )
    }

    fn part_b(&self) -> Option<i64> {
        Some(
            self.machines
                .iter()
                .flat_map(|m| Self::solve_machine(m, 10000000000000))
                .sum(),
        )
    }
}

impl Day {
    fn solve_machine(machine: &Machine, offset: i64) -> Option<i64> {
        let prize = machine.prize + Vector::new(offset, offset);
        let a = (prize.x * machine.b.y - prize.y * machine.b.x)
            / (machine.a.x * machine.b.y - machine.a.y * machine.b.x);
        let b = (prize.y - a * machine.a.y) / machine.b.y;

        match machine.a * a + machine.b * b {
            sum if sum == prize => Some(3 * a + b),
            _ => None,
        }
    }
}

crate::solution::test_solution!();
