use std::collections::HashMap;

use itertools::Itertools;
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
        Some(self.machines.iter().map(Machine::min_presses).sum())
    }

    fn part_b(&self) -> Option<usize> {
        Some(self.machines.iter().map(Machine::solve).sum())
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

    fn coefficients(&self) -> Vec<Vec<isize>> {
        self.schematics
            .iter()
            .map(|button| {
                let mut coefficients = vec![0; self.requirements.len()];
                for &light in button {
                    coefficients[light] = 1;
                }
                coefficients
            })
            .collect()
    }

    fn patterns(&self) -> HashMap<Vec<isize>, usize> {
        let mut patterns = HashMap::new();
        let coefficients = self.coefficients();

        for pattern_len in 0..=self.buttons.len() {
            for buttons in (0..self.buttons.len()).combinations(pattern_len) {
                let mut pattern = vec![0; self.requirements.len()];

                for &i in &buttons {
                    for (p, &c) in pattern.iter_mut().zip(&coefficients[i]) {
                        *p += c;
                    }
                }

                patterns.entry(pattern).or_insert(pattern_len);
            }
        }

        patterns
    }

    fn solve(&self) -> usize {
        fn helper(
            cache: &mut HashMap<Vec<isize>, usize>,
            costs: &HashMap<Vec<isize>, usize>,
            goal: &[isize],
        ) -> usize {
            if let Some(&previous) = cache.get(goal) {
                return previous;
            }

            if goal.iter().all(|&n| n == 0) {
                return 0;
            }

            // For some reason, usize::MAX / 2 does not work, but 3 does.
            // I use 4 to be safe, in case other input files don't work.
            let mut answer = usize::MAX / 4;
            for (pattern, &cost) in costs {
                if pattern
                    .iter()
                    .zip(goal)
                    .all(|(a, b)| a <= b && a % 2 == b % 2)
                {
                    let goal: Vec<_> = pattern.iter().zip(goal).map(|(a, b)| (b - a) / 2).collect();
                    let sub = helper(cache, costs, &goal);
                    answer = answer.min(cost + 2 * sub);
                }
            }

            cache.insert(goal.to_vec(), answer);
            answer
        }

        let goal: Vec<_> = self.requirements.iter().map(|&n| n as isize).collect();
        helper(&mut HashMap::new(), &self.patterns(), &goal)
    }
}

utils::solution::test_solution!(aoc2025, day10);
