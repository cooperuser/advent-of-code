use utils::prelude::*;
use z3::{Solver, ast::Int};

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    machines: Vec<Machine>,
}

#[derive(Debug)]
struct Machine {
    lights: usize,
    buttons: Vec<usize>,
    schematics: Vec<Vec<usize>>,
    requirements: Vec<usize>,
}

impl Solution<usize, usize> for Day {
    fn meta() -> Meta<usize, usize> {
        Meta::<usize, usize> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 7,
            answer_b: 33,
        }
    }

    fn new(raw: Vec<Rc<str>>) -> Self {
        let mut machines = Vec::new();

        for line in &raw {
            let parts: Vec<_> = line.split_whitespace().collect();
            let raw_lights: Vec<_> = parts[0].chars().collect();
            let raw_requirements = parts[parts.len() - 1];
            let raw_requirements = &raw_requirements[1..raw_requirements.len() - 1];
            let raw_buttons: Vec<_> = parts.iter().skip(1).take(parts.len() - 2).collect();

            let mut lights = 0;
            for &light in raw_lights[1..raw_lights.len() - 1].iter().rev() {
                lights <<= 1;
                if light == '#' {
                    lights += 1;
                }
            }

            let mut requirements = Vec::new();
            for requirement in raw_requirements.split(',') {
                requirements.push(requirement.parse().unwrap());
            }

            let mut buttons = Vec::new();
            let mut schematics = Vec::new();
            for button in raw_buttons {
                buttons.push(
                    button[1..button.len() - 1]
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .map(|n| 2usize.pow(n))
                        .sum(),
                );
                schematics.push(
                    button[1..button.len() - 1]
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .collect(),
                );
            }

            machines.push(Machine {
                lights,
                buttons,
                schematics,
                requirements,
            })
        }

        Self { raw, machines }
    }

    fn part_a(&self) -> Option<usize> {
        Some(self.machines.iter().map(Machine::min_presses).sum())
    }

    fn part_b(&self) -> Option<usize> {
        let mut sum = 0;

        for (m, machine) in self.machines.iter().enumerate() {
            sum += machine.solve();
            println!("{:?}", m);
        }

        Some(sum)
        // Some(self.machines.iter().map(Machine::solve).sum())
    }
}

impl Machine {
    fn min_presses(&self) -> usize {
        (1..(1 << self.buttons.len()))
            .filter(|&pressed| {
                let lights = self
                    .buttons
                    .iter()
                    .enumerate()
                    .filter_map(|(shift, button)| (pressed & (1 << shift) != 0).then_some(button))
                    .fold(0, |lights, button| lights ^ button);

                lights == self.lights
            })
            .map(usize::count_ones)
            .min()
            .unwrap() as usize
    }

    fn solve(&self) -> usize {
        let solver = Solver::new();
        let null = Int::fresh_const("null");
        solver.assert(null.eq(0));
        let mut buttons = Vec::new();

        for b in 0..self.buttons.len() {
            let button = Int::fresh_const(format!("button_{b}").as_str());
            solver.assert(button.ge(0));
            buttons.push(button);
        }

        for (r, &requirement) in self.requirements.iter().enumerate() {
            let mut equation = null.clone();
            for (b, button) in self.schematics.iter().enumerate() {
                if button.contains(&r) {
                    equation += &buttons[b];
                }
            }
            solver.assert(equation.eq(requirement as i64));
        }

        let mut min: usize = usize::MAX;
        for solution in solver.solutions(buttons, false) {
            let sum = solution
                .iter()
                .map(Int::as_u64)
                .map(Option::unwrap)
                .sum::<u64>() as usize;

            min = min.min(sum);
        }

        min
    }
}

utils::solution::test_solution!(aoc2025, day10);
