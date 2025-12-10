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
        let mut total_presses = 0;

        for machine in &self.machines {
            let mut queue = VecDeque::from([(0, 0, None)]);
            while let Some((presses, lights, last)) = queue.pop_front() {
                if lights == machine.lights {
                    total_presses += presses;
                    break;
                }

                for (i, button) in machine.buttons.iter().enumerate() {
                    if Some(i) == last {
                        continue;
                    }

                    queue.push_back((presses + 1, lights ^ button, Some(i)));
                }
            }
        }

        Some(total_presses)
    }

    fn part_b(&self) -> Option<usize> {
        let mut total_presses = 0;

        for machine in &self.machines {
            let mut queue = VecDeque::from([(
                vec![0; machine.buttons.len()],
                0,
                vec![0; machine.requirements.len()],
            )]);
            let mut state: Option<(Vec<usize>, Vec<usize>)> = None;
            while let Some((presses, lights, reqs)) = queue.pop_front() {
                if lights == machine.lights {
                    state = Some((presses, reqs));
                    break;
                }

                for (index, (button, schematics)) in
                    machine.buttons.iter().zip(&machine.schematics).enumerate()
                {
                    let mut requirements = reqs.clone();
                    for &i in schematics {
                        requirements[i] += 1;
                    }
                    let presses = presses
                        .iter()
                        .enumerate()
                        .map(|(i, &b)| match i == index {
                            true => b + 1,
                            false => b,
                        })
                        .collect();
                    queue.push_back((presses, lights ^ button, requirements));
                }
            }

            let state = state.unwrap();
            let presses = state.0.iter().filter(|&&n| n > 0).count();
            let mut queue = VecDeque::from([(state.0, presses, state.1)]);
            while let Some((pressed, presses, reqs)) = queue.pop_front() {
                if reqs == machine.requirements {
                    total_presses += presses;
                    break;
                }

                for schematics in &machine.schematics {
                    let mut pressed = pressed.clone();
                    let mut reqs = reqs.clone();
                    for (i, &s) in schematics.iter().enumerate() {
                        pressed[i] += 2;
                        reqs[s] += 2;
                    }
                    queue.push_back((pressed, presses + 2, reqs));
                }
            }
        }

        Some(total_presses)
    }
}

utils::solution::test_solution!(aoc2025, day10);
