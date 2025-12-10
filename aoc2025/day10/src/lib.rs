use std::{collections::VecDeque, time::Instant};

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
            let mut queue = VecDeque::from([(0, 0, vec![false; machine.buttons.len()])]);
            while let Some((presses, lights, pressed)) = queue.pop_front() {
                if lights == machine.lights {
                    total_presses += presses;
                    break;
                }

                for (index, button) in machine.buttons.iter().enumerate() {
                    if pressed[index] {
                        continue;
                    }

                    let pressed = pressed
                        .iter()
                        .enumerate()
                        .map(|(i, &p)| match i == index {
                            true => true,
                            false => p,
                        })
                        .collect();

                    queue.push_back((presses + 1, lights ^ button, pressed));
                }
            }
        }

        Some(total_presses)
    }

    fn part_b(&self) -> Option<usize> {
        let start = Instant::now();
        let mut total_presses = 0;

        for (m, machine) in self.machines.iter().enumerate() {
            let mut queue = VecDeque::from([(
                0,
                vec![0; machine.buttons.len()],
                vec![0; machine.requirements.len()],
            )]);
            let mut last_presses = 0;
            while let Some((presses, pressed, reqs)) = queue.pop_front() {
                if presses > last_presses {
                    // println!("{presses} ({} in the queue)", queue.len());
                    last_presses = presses;
                } else if reqs == machine.requirements {
                    // println!("Adding {presses}");
                    total_presses += presses;
                    break;
                }

                if reqs.iter().zip(&machine.requirements).any(|(a, b)| a > b) {
                    continue;
                }

                for (index, schematic) in machine.schematics.iter().enumerate() {
                    let offset = 1;
                    let pressed = pressed
                        .iter()
                        .enumerate()
                        .map(|(i, &n)| match i == index {
                            true => n + offset,
                            false => n,
                        })
                        .collect();

                    let mut reqs = reqs.clone();
                    for &l in schematic {
                        reqs[l] += offset;
                    }

                    queue.push_back((presses + offset, pressed, reqs));
                }
            }
            println!("{}/{}", m + 1, self.machines.len());
        }

        println!("{:?}", start.elapsed());

        Some(total_presses)
    }
}

utils::solution::test_solution!(aoc2025, day10);
