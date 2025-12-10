use std::collections::VecDeque;

use utils::prelude::*;

pub struct Day {
    #[allow(dead_code)]
    raw: Vec<Rc<str>>,
    machines: Vec<Machine>,
}

#[derive(Debug)]
struct Machine {
    lights: usize,
    buttons: Vec<usize>,
    requirements: Vec<i64>,
}

impl Solution<i64, i64> for Day {
    fn meta() -> Meta<i64, i64> {
        Meta::<i64, i64> {
            input: include_str!("input.txt").to_string(),
            sample_a: include_str!("input_sample.txt").to_string(),
            sample_b: include_str!("input_sample.txt").to_string(),
            answer_a: 7,
            answer_b: 0,
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

            let mut buttons: Vec<usize> = Vec::new();
            for button in raw_buttons {
                buttons.push(
                    button[1..button.len() - 1]
                        .split(',')
                        .map(|n| n.parse().unwrap())
                        .map(|n| 2usize.pow(n))
                        .sum(),
                );
            }

            machines.push(Machine {
                lights,
                buttons,
                requirements,
            })
        }

        Self { raw, machines }
    }

    fn part_a(&self) -> Option<i64> {
        let mut presses = 0;
        for machine in &self.machines {
            let mut queue = VecDeque::new();
            queue.push_back((0, 0));
            while let Some(state) = queue.pop_front() {
                if state.1 == machine.lights {
                    presses += state.0;
                    break;
                }

                for button in &machine.buttons {
                    queue.push_back((state.0 + 1, state.1 ^ button));
                }
            }
        }
        Some(presses)
    }

    fn part_b(&self) -> Option<i64> {
        None
    }
}

utils::solution::test_solution!(aoc2025, day10);
